<p align=center><img src=".github/bitmap.png" style="width:128px;"/></p>
<h1 align=center>Open Witness Library</h1>

<p align=center><b>Open Witness Library</b> is a program to read publications that carries <i>God's true name</i>.</p>
<p align=center>
    <a href="https://github.com/orangethewell/open-witness-library/releases/latest"><img src="https://img.shields.io/github/v/tag/orangethewell/open-witness-library?include_prereleases&sort=semver&style=plastic&label=Latest%20version" alt="tag-version"/></a>
    <img src="https://img.shields.io/github/downloads/orangethewell/open-witness-library/total?style=plastic&label=Downloads" alt="downloads-count"/>
    <img src="https://img.shields.io/github/actions/workflow/status/orangethewell/open-witness-library/ci.yml" alt="build-status">
</p>

This program is a alternative for **JW Library** app[^1] for OSes that doesn't support the app or had lost support through the years. You can read `.jwpub` files and study Bible.

## Features

This program still is a work-in-progress, so it isn't full compatible with all publications and publication's features. You can track the features present on the program [here](https://github.com/orangethewell/open-witness-library/issues/5).

## Installation

See [Building](#building) for more information.

## Building

For testing the app, you need to setup a *Rust* development environment on your operational system. After installing every dependency and following the prerequisites below, you should install [Rustup](https://rustup.rs/) through the official site or from your package manager and download the latest Rust language version.

### Windows

For Windows, see [Windows prerequisites](https://tauri.app/start/prerequisites/#windows).

### MacOS (Catalina (10.15) and later)

For MacOS, you will need to install **CLang** and *MacOS development dependencies*. To do this, run the following command in your terminal:

```sh
xcode-select --install
```

### Linux

For Linux, you will need to install some packages in order to work, see the instruction on how to install based in your [Linux distro](https://tauri.app/start/prerequisites/#linux).

### UI Setup

After installing the dependencies, you'll need to setup Node and NPM for compiling the application UI and serving it locally.

#### Install Node.js and npm

1. Download and install Node.js, which includes npm, from the [official website](https://nodejs.org/). Choose the LTS version for stability.
2. Verify the installation by running the following commands in your terminal or command prompt:

```sh
node -v
npm -v
```

Both commands should return version numbers.

Once npm is installed, navigate to the project root directory and install the dependencies for UI compilation:

```sh
cd ui
npm install
```

after installation, you are free to run `cargo tauri dev` for running the application.

### Running on mobile (NOTE: Not well supported)

*Open Witness Library* also runs on **Android** and **iOS**, but it isn't fully supported. In the current state, the app will probably run fine, but it not tested, so you can have some problems, glitches and bugs. If you want to test it out on Android, you need to add the Android targets to your rustup installation by running the following command:

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

You will need to install [**Android Studio**](https://developer.android.com/studio) and setup it to your system.

Install the **Android SDK** and **NDK**.
You can use the SDK Manager in Android Studio to install:

1. Android SDK Platform
2. Android SDK Platform-Tools
3. NDK (Side by side)
4. Android SDK Build-Tools
5. Android SDK Command-line Tools

You will need to configure JDK too. Android Studio includes a JDK, you will just need to add it to your environment variables:

```ps
# On windows
[System.Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\Android\Android Studio\jbr", "User")
```

```sh
# On MacOS
export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"

# On Linux
# Note that on older Android Studio installations the 
# jbr directory may still be called jre
export JAVA_HOME=/opt/android-studio/jbr

```

and finally, you need to set `ANDROID_HOME` and `NDK_HOME` environment variables:

```ps
# On Windows (change NDK_VERSION to the version located in the path without brackets)
[System.Environment]::SetEnvironmentVariable("ANDROID_HOME", "$env:LocalAppData\Android\Sdk", "User")
[System.Environment]::SetEnvironmentVariable("NDK_HOME", "$env:LocalAppData\Android\Sdk\ndk\{NDK_VERSION}", "User")
```

```sh
# On Linux (change NDK_VERSION to the version located in the path without brackets)
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/{NDK_VERSION}"

# On MacOS (change NDK_VERSION to the version located in the path without brackets)
export ANDROID_HOME="$HOME/Library/Android/sdk"
export NDK_HOME="$ANDROID_HOME/ndk/{NDK_VERSION}"
```

To run in Android, you can run the following command:

```sh
cargo tauri android dev
```

#### Running in iOS

You can also run on iOS, but you will need a MacOS device. I don't know if it will run since I didn't have an iOS device or a MacOS computer/laptop, but you can test it. First, make sure **Xcode** is properly installed. Then, you should add the iOS target to your MacOS rustup installation:

```sh
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
```

Then you can run your application on iOS with the following commmand:

```sh
cargo tauri ios dev
```

## Contributing

Pull requests are welcome. For translations, take a look on `i18n/` folder. For major changes, please open an issue first to discuss what you would like to change.

## Disclaimer

This program don't have any relationship with **JW Library** developers or the *Watch Tower Bible and Tract Society of Pennsylvania*. This is a program maintened by the community. Any bug you have using this program should be issued [there](https://github.com/orangethewell/open-witness-library/issues).

If your computer or mobile phone runs JW Library, please prefer to use the official app instead of this program.

[^1]: [JW Library](https://www.jw.org/en/online-help/jw-library/) is a registered trademark of *Watch Tower Bible and Tract Society of Pennsylvania*.
