# Desktop Application

C#/.NET 11 + Avalonia UI desktop application for Windows 11 and macOS.

## Structure

```
desktop/
├── Vega.Desktop/         # Main Avalonia application
│   ├── Assets/          # Icons, images, cursors, fonts, animations
│   ├── Themes/          # Light, dark, high-contrast themes
│   ├── Controls/        # Custom controls (Canvas, Panels, Toolbars, Dialogs)
│   ├── Views/           # XAML views (Main, Dialogs, Panels, Welcome, Settings)
│   ├── ViewModels/      # MVVM view models
│   ├── Services/        # Application services (Files, Preferences, Cloud, Recent)
│   ├── Converters/      # Value converters
│   ├── Behaviors/       # XAML behaviors
│   ├── Styles/          # Custom styles
│   └── Resources/       # Resource dictionaries
├── Vega.Application/     # Application logic and workflows
│   ├── Commands/        # Command definitions (Layer, Selection, Transform, etc.)
│   ├── Workflows/       # High-level workflows (Open, Save, Export, Import, Batch)
│   ├── Services/        # Application services
│   ├── Interfaces/      # Service interfaces
│   ├── Models/          # Domain models
│   ├── Validation/      # Validation logic
│   ├── Exceptions/      # Custom exceptions
│   ├── Converters/      # Data converters
│   └── Mappers/         # Object mappers
├── Vega.Presentation/    # Presentation layer
│   ├── ViewModels/      # Shared view models
│   ├── Models/          # Presentation models
│   ├── Design/          # Design-time data
│   ├── Behaviors/       # Behaviors
│   ├── Converters/      # Converters
│   └── Validation/      # Validation
├── Vega.NativeInterop/   # Rust core interop
│   ├── Bindings/        # P/Invoke/LibraryImport declarations
│   ├── Wrappers/        # Safe C# wrappers for native APIs
│   ├── Models/          # Interop data models
│   ├── Marshaling/      # Marshaling utilities
│   ├── Interop/         # Interop utilities
│   └── Tests/           # Interop tests
└── Vega.Tests/          # Desktop application tests
    ├── Unit/            # Unit tests
    ├── Integration/     # Integration tests
    ├── UI/              # UI tests
    ├── Fixtures/        # Test fixtures
    └── Mocks/           # Test mocks
```

## Build

```bash
dotnet build VEGA.sln -c Release
```

## Run

```bash
dotnet run --project Vega.Desktop/Vega.Desktop.csproj
```

## Test

```bash
dotnet test VEGA.sln -c Release
```

## Documentation

See [../../docs/guides/dotnet-development.md](../../docs/guides/dotnet-development.md) for C#/.NET development guidelines.
