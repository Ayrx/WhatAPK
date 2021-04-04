use abxml::{decoder::Decoder, model::Element};
use anyhow::{anyhow, ensure, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug)]
pub struct AndroidManifest {
    pub package_name: String,
    pub permissions: Vec<Permission>,
    pub api_level: u8,
    pub activities: Vec<String>,
    pub services: Vec<String>,
    pub receivers: Vec<String>,
    pub providers: Vec<String>,
}

impl AndroidManifest {
    pub fn parse(resources_file: &[u8], manifest_file: &[u8]) -> Result<Self> {
        let decoder = Decoder::from_buffer(resources_file).map_err(|e| e.compat())?;

        let visitor = decoder
            .xml_visitor(&manifest_file)
            .map_err(|e| e.compat())?;

        let root = visitor.get_root().as_ref().unwrap();
        ensure!(
            root.get_tag().get_name().as_str() == "manifest",
            "AndroidManifest: missing `manifest` tag"
        );

        let attr = root.get_attributes();

        let package_name = attr
            .get("package")
            .ok_or_else(|| anyhow!("AndroidManifest: missing `package` attribute"))?
            .to_owned();

        let api_level = attr
            .get("platformBuildVersionCode")
            .ok_or_else(|| {
                anyhow!("AndroidManifest: missing `platformBuildVersionCode` attribute")
            })?
            .parse::<u8>()?;

        let mut activities = Vec::new();
        let mut permissions = Vec::new();
        let mut services = Vec::new();
        let mut receivers = Vec::new();
        let mut providers = Vec::new();

        for child in root.get_children() {
            match child.get_tag().get_name().as_str() {
                "uses-permission" => {
                    if let Some(p) = Self::parse_permissions(child)? {
                        permissions.push(p);
                    }
                }
                "application" => {
                    for c in child.get_children() {
                        match c.get_tag().get_name().as_str() {
                            "activity" => activities.push(Self::parse_activity(c)?),
                            "service" => services.push(Self::parse_service(c)?),
                            "receiver" => receivers.push(Self::parse_receiver(c)?),
                            "provider" => providers.push(Self::parse_provider(c)?),
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }

        Ok(AndroidManifest {
            package_name,
            permissions,
            api_level,
            activities,
            services,
            receivers,
            providers,
        })
    }

    fn parse_permissions(element: &Element) -> Result<Option<Permission>> {
        let permission = element
            .get_attributes()
            .get("android:name")
            .ok_or_else(|| anyhow!("AndroidManifest: malformed `uses-permission` element"))?;

        Ok(Permission::from_manifest_string(permission))
    }

    fn parse_activity(element: &Element) -> Result<String> {
        Ok(element
            .get_attributes()
            .get("android:name")
            .ok_or_else(|| anyhow!("AndroidManifest: malformed `activity` element"))?
            .to_owned())
    }

    fn parse_service(element: &Element) -> Result<String> {
        Ok(element
            .get_attributes()
            .get("android:name")
            .ok_or_else(|| anyhow!("AndroidManifest: malformed `service` element"))?
            .to_owned())
    }

    fn parse_receiver(element: &Element) -> Result<String> {
        Ok(element
            .get_attributes()
            .get("android:name")
            .ok_or_else(|| anyhow!("AndroidManifest: malformed `receiver` element"))?
            .to_owned())
    }

    fn parse_provider(element: &Element) -> Result<String> {
        Ok(element
            .get_attributes()
            .get("android:name")
            .ok_or_else(|| anyhow!("AndroidManifest: malformed `provider` element"))?
            .to_owned())
    }

    pub fn print(&self) {
        println!("========================================");
        println!("[+] Package: {}", self.package_name);
        println!("[+] API level: {}", self.api_level);

        println!("[+] Permissions:");
        for i in &self.permissions {
            println!("{}", i);
        }

        println!("[+] Activities:");
        for i in &self.activities {
            println!("{}", i);
        }

        println!("[+] Services:");
        for i in &self.services {
            println!("{}", i);
        }

        println!("[+] Receivers:");
        for i in &self.receivers {
            println!("{}", i);
        }

        println!("[+] Providers:");
        for i in &self.providers {
            println!("{}", i);
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Display, EnumString)]
pub enum Permission {
    ACCEPT_HANDOVER,
    ACCESS_BACKGROUND_LOCATION,
    ACCESS_CHECKIN_PROPERTIES,
    ACCESS_COARSE_LOCATION,
    ACCESS_FINE_LOCATION,
    ACCESS_LOCATION_EXTRA_COMMANDS,
    ACCESS_MEDIA_LOCATION,
    ACCESS_NETWORK_STATE,
    ACCESS_NOTIFICATION_POLICY,
    ACCESS_WIFI_STATE,
    ACTIVITY_RECOGNITION,
    ADD_VOICEMAIL,
    ANSWER_PHONE_CALLS,
    BATTERY_STATS,
    BIND_ACCESSIBILITY_SERVICE,
    BIND_APPWIDGET,
    BIND_AUTOFILL_SERVICE,
    BIND_CALL_REDIRECTION_SERVICE,
    BIND_CARRIER_MESSAGING_CLIENT_SERVICE,
    BIND_CARRIER_MESSAGING_SERVICE,
    BIND_CARRIER_SERVICES,
    BIND_CHOOSER_TARGET_SERVICE,
    BIND_COMPANION_DEVICE_SERVICE,
    BIND_CONDITION_PROVIDER_SERVICE,
    BIND_CONTROLS,
    BIND_DEVICE_ADMIN,
    BIND_DREAM_SERVICE,
    BIND_INCALL_SERVICE,
    BIND_INPUT_METHOD,
    BIND_MIDI_DEVICE_SERVICE,
    BIND_NFC_SERVICE,
    BIND_NOTIFICATION_LISTENER_SERVICE,
    BIND_PRINT_SERVICE,
    BIND_QUICK_ACCESS_WALLET_SERVICE,
    BIND_QUICK_SETTINGS_TILE,
    BIND_REMOTEVIEWS,
    BIND_SCREENING_SERVICE,
    BIND_TELECOM_CONNECTION_SERVICE,
    BIND_TEXT_SERVICE,
    BIND_TV_INPUT,
    BIND_VISUAL_VOICEMAIL_SERVICE,
    BIND_VOICE_INTERACTION,
    BIND_VPN_SERVICE,
    BIND_VR_LISTENER_SERVICE,
    BIND_WALLPAPER,
    BLUETOOTH,
    BLUETOOTH_ADMIN,
    BLUETOOTH_PRIVILEGED,
    BODY_SENSORS,
    BROADCAST_PACKAGE_REMOVED,
    BROADCAST_SMS,
    BROADCAST_STICKY,
    BROADCAST_WAP_PUSH,
    CALL_COMPANION_APP,
    CALL_PHONE,
    CALL_PRIVILEGED,
    CAMERA,
    CAPTURE_AUDIO_OUTPUT,
    CHANGE_COMPONENT_ENABLED_STATE,
    CHANGE_CONFIGURATION,
    CHANGE_NETWORK_STATE,
    CHANGE_WIFI_MULTICAST_STATE,
    CHANGE_WIFI_STATE,
    CLEAR_APP_CACHE,
    CONTROL_LOCATION_UPDATES,
    DELETE_CACHE_FILES,
    DELETE_PACKAGES,
    DIAGNOSTIC,
    DISABLE_KEYGUARD,
    DUMP,
    EXPAND_STATUS_BAR,
    FACTORY_TEST,
    FOREGROUND_SERVICE,
    GET_ACCOUNTS,
    GET_ACCOUNTS_PRIVILEGED,
    GET_PACKAGE_SIZE,
    GET_TASKS,
    GLOBAL_SEARCH,
    HIDE_OVERLAY_WINDOWS,
    HIGH_SAMPLING_RATE_SENSORS,
    INSTALL_LOCATION_PROVIDER,
    INSTALL_PACKAGES,
    INSTALL_SHORTCUT,
    INSTANT_APP_FOREGROUND_SERVICE,
    INTERACT_ACROSS_PROFILES,
    INTERNET,
    KILL_BACKGROUND_PROCESSES,
    LOADER_USAGE_STATS,
    LOCATION_HARDWARE,
    MANAGE_DOCUMENTS,
    MANAGE_EXTERNAL_STORAGE,
    MANAGE_ONGOING_CALLS,
    MANAGE_OWN_CALLS,
    MASTER_CLEAR,
    MEDIA_CONTENT_CONTROL,
    MODIFY_AUDIO_SETTINGS,
    MODIFY_PHONE_STATE,
    MOUNT_FORMAT_FILESYSTEMS,
    MOUNT_UNMOUNT_FILESYSTEMS,
    NFC,
    NFC_PREFERRED_PAYMENT_INFO,
    NFC_TRANSACTION_EVENT,
    PACKAGE_USAGE_STATS,
    PERSISTENT_ACTIVITY,
    PROCESS_OUTGOING_CALLS,
    QUERY_ALL_PACKAGES,
    READ_CALENDAR,
    READ_CALL_LOG,
    READ_CONTACTS,
    READ_EXTERNAL_STORAGE,
    READ_INPUT_STATE,
    READ_LOGS,
    READ_PHONE_NUMBERS,
    READ_PHONE_STATE,
    READ_PRECISE_PHONE_STATE,
    READ_SMS,
    READ_SYNC_SETTINGS,
    READ_SYNC_STATS,
    READ_VOICEMAIL,
    REBOOT,
    RECEIVE_BOOT_COMPLETED,
    RECEIVE_MMS,
    RECEIVE_SMS,
    RECEIVE_WAP_PUSH,
    RECORD_AUDIO,
    REORDER_TASKS,
    REQUEST_COMPANION_PROFILE_WATCH,
    REQUEST_COMPANION_RUN_IN_BACKGROUND,
    REQUEST_COMPANION_USE_DATA_IN_BACKGROUND,
    REQUEST_DELETE_PACKAGES,
    REQUEST_IGNORE_BATTERY_OPTIMIZATIONS,
    REQUEST_INSTALL_PACKAGES,
    REQUEST_OBSERVE_COMPANION_DEVICE_PRESENCE,
    REQUEST_PASSWORD_COMPLEXITY,
    RESTART_PACKAGES,
    CHEDULE_EXACT_ALARM,
    END_RESPOND_VIA_MESSAGE,
    END_SMS,
    ET_ALARM,
    ET_ALWAYS_FINISH,
    ET_ANIMATION_SCALE,
    ET_DEBUG_APP,
    ET_PREFERRED_APPLICATIONS,
    ET_PROCESS_LIMIT,
    ET_TIME,
    ET_TIME_ZONE,
    ET_WALLPAPER,
    ET_WALLPAPER_HINTS,
    IGNAL_PERSISTENT_PROCESSES,
    MS_FINANCIAL_TRANSACTIONS,
    TART_VIEW_PERMISSION_USAGE,
    TATUS_BAR,
    YSTEM_ALERT_WINDOW,
    TRANSMIT_IR,
    UNINSTALL_SHORTCUT,
    UPDATE_DEVICE_STATS,
    USE_BIOMETRIC,
    USE_FINGERPRINT,
    USE_FULL_SCREEN_INTENT,
    USE_ICC_AUTH_WITH_DEVICE_IDENTIFIER,
    USE_SIP,
    VIBRATE,
    WAKE_LOCK,
    WRITE_APN_SETTINGS,
    WRITE_CALENDAR,
    WRITE_CALL_LOG,
    WRITE_CONTACTS,
    WRITE_EXTERNAL_STORAGE,
    WRITE_GSERVICES,
    WRITE_SECURE_SETTINGS,
    WRITE_SETTINGS,
    WRITE_SYNC_SETTINGS,
    WRITE_VOICEMAIL,
}

impl Permission {
    pub fn from_manifest_string(s: &str) -> Option<Permission> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"android\.permission\.([A-Z_]*)").unwrap();
        }

        match RE.captures(s) {
            Some(m) => Self::from_str(m.get(1).unwrap().as_str()).ok(),
            None => None,
        }
    }
}
