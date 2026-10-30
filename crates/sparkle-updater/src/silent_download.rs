//! Persists the automatic downloads setting that a silent download turned on
//! for one update cycle, so the next launch restores it if the app quit first.

use objc2_foundation::{NSString, NSUserDefaults};

const AUTOMATICALLY_UPDATE_KEY: &str = "SUAutomaticallyUpdate";
const RESTORE_KEY: &str = "SparkleUpdaterRestoreAutomaticDownloads";

pub(crate) fn remember(enabled: bool) {
    NSUserDefaults::standardUserDefaults()
        .setBool_forKey(enabled, &NSString::from_str(RESTORE_KEY));
}

pub(crate) fn forget() {
    NSUserDefaults::standardUserDefaults().removeObjectForKey(&NSString::from_str(RESTORE_KEY));
}

/// Runs before Sparkle reads its settings.
pub(crate) fn restore_interrupted() {
    let defaults = NSUserDefaults::standardUserDefaults();
    let key = NSString::from_str(RESTORE_KEY);
    if defaults.objectForKey(&key).is_none() {
        return;
    }
    let enabled = defaults.boolForKey(&key);
    defaults.setBool_forKey(enabled, &NSString::from_str(AUTOMATICALLY_UPDATE_KEY));
    defaults.removeObjectForKey(&key);
}
