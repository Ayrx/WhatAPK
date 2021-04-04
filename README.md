# WhatAPK

`whatapk` is a command line utility that attempts to fingerprint frameworks or
libraries used in an Android APK. As opposed to [APKiD][apkid], `whatapk`
attempts to fingerprint general purpose libraries and frameworks instead of
focusing on packers or obfuscators.

`whatapk` also parses the `AndroidManifest.xml` file to identify useful
information about the APK, such as the package identifier, a list of required
permissions and a list of components implemented by the application.

## Usage

```
➜  ./whatapk --help
whatapk 0.1.0
Terry Chia <terrycwk1994@gmail.com>


USAGE:
    whatapk [OPTIONS] [APK]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <FILE>    Write results to a JSON file.

ARGS:
    <APK>    APK file to analyze.
```

Example output:

```
➜  ./whatapk com.orgname.SWNewsReader.apk
========================================
[+] Package: com.orgname.SWNewsReader
[+] API level: 26
[+] Permissions:
READ_PHONE_STATE
ACCESS_NETWORK_STATE
INTERNET
ACCESS_NETWORK_STATE
WRITE_EXTERNAL_STORAGE
[+] Activities:
com.orgname.SWNewsReader.SWNewsReader
[+] Services:
sync.kony.com.syncv2library.Android.Network.HttpService
[+] Receivers:
[+] Providers:
com.orgname.SWNewsReader.SWNewsReaderSearchSuggestionProvider
========================================
[+] Kony Visualizer detected
[+] Files:
assets/application.properties
assets/js/common-jslibs.kfm
assets/js/startup.js
assets/pluginversions.properties
lib/armeabi-v7a/libkonyjsvm.so
```

The `-o` option can be used to save the results to a JSON file for later
processing.

[apkid]: https://github.com/rednaga/APKiD
