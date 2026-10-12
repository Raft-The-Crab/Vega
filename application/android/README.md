# Android Application

Kotlin + Jetpack Compose Android application for Android 11+.

## Structure

```
android/
├── app/
│   ├── src/main/java/com/vega/
│   │   ├── app/           # Application class and main entry point
│   │   ├── ui/            # UI layer
│   │   │   ├── theme/     # Material theme and styling
│   │   │   ├── screens/   # Compose screens
│   │   │   ├── components/ # Reusable UI components
│   │   │   └── viewmodels/ # Screen view models
│   │   ├── domain/        # Domain layer (use cases)
│   │   ├── data/          # Data layer
│   │   │   ├── local/     # Local data sources
│   │   │   └── remote/    # Remote data sources
│   │   ├── di/            # Dependency injection
│   │   └── util/          # Utilities
│   ├── src/main/res/
│   │   ├── drawable/      # Drawable resources
│   │   ├── layout/        # Layout resources
│   │   ├── values/        # Values resources
│   │   ├── mipmap/        # Mipmap resources
│   │   ├── raw/           # Raw resources
│   │   └── xml/           # XML resources
│   ├── src/test/          # Unit tests
│   └── src/androidTest/   # Instrumented tests
└── build.gradle.kts       # Module build configuration
```

## Build

```bash
./gradlew build
```

## Run

```bash
./gradlew installDebug
```

## Test

```bash
./gradlew test
./gradlew connectedAndroidTest
```

## Documentation

See [../../docs/guides/android-development.md](../../docs/guides/android-development.md) for Android development guidelines.

## Status

The Android application is currently in the planning phase. Desktop (Windows) is the initial implementation priority. Android development will begin after the desktop foundation is stable.
