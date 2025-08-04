{ pkgs, lib, config, inputs, ... }:

{
  # Java development environment for LibGDX project
  languages.java = {
    enable = true;
    jdk.package = pkgs.jdk8; # LibGDX 1.9.7 works best with Java 8
    gradle.enable = true;
    maven.enable = true;
  };

  # Development packages
  packages = with pkgs; [
    git
    git-lfs
    gradle # Use latest gradle, wrapper will handle version
    maven
    jdt-language-server # Java LSP for IDE support
    google-java-format # Code formatting
    checkstyle # Code style checking
    
    # X11 and OpenGL libraries for LWJGL/LibGDX
    xorg.libX11
    xorg.libXext
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libXxf86vm # Missing library causing the error
    xorg.libXinerama
    libGL
    libGLU
    openal
    alsa-lib
    mesa
  ];

  # Environment variables
  env = {
    JAVA_HOME = "${pkgs.jdk8}/lib/openjdk";
    GRADLE_OPTS = "-Xmx2g -Dorg.gradle.daemon=true";
    # LibGDX/LWJGL native library path with all required X11 libraries
    LD_LIBRARY_PATH = lib.makeLibraryPath [
      pkgs.libGL
      pkgs.libGLU
      pkgs.openal
      pkgs.alsa-lib
      pkgs.xorg.libX11
      pkgs.xorg.libXext
      pkgs.xorg.libXcursor
      pkgs.xorg.libXrandr
      pkgs.xorg.libXi
      pkgs.xorg.libXxf86vm
      pkgs.xorg.libXinerama
      pkgs.mesa
    ];
  };

  # Development scripts
  scripts = {
    build.exec = ''
      echo "Building Astraria project..."
      ./gradlew build
    '';
    
    run.exec = ''
      echo "Running Astraria..."
      ./gradlew desktop:run
    '';
    
    clean.exec = ''
      echo "Cleaning build artifacts..."
      ./gradlew clean
    '';
    
    format.exec = ''
      echo "Formatting Java code..."
      find . -name "*.java" | grep -v build | xargs google-java-format --replace
    '';
    
    check.exec = ''
      echo "Running code quality checks..."
      ./gradlew check
    '';
  };

  # Shell setup
  enterShell = ''
    echo "🚀 Astraria Development Environment"
    echo "Java: $(java -version 2>&1 | head -1)"
    echo "Gradle: $(gradle --version | grep Gradle)"
    echo ""
    echo "Available commands:"
    echo "  build  - Build the project"
    echo "  run    - Run the game"
    echo "  clean  - Clean build artifacts"
    echo "  format - Format Java code"
    echo "  check  - Run quality checks"
    echo ""
    echo "LibGDX native libraries configured for Linux"
    
    # Make gradlew executable if it isn't
    chmod +x gradlew
  '';

  # Git hooks for code quality (disabled for now)
  # git-hooks.hooks.java-format = {
  #   enable = true;
  #   name = "java-format";
  #   entry = "${pkgs.google-java-format}/bin/google-java-format --replace";
  #   files = "\\.java$";
  #   language = "system";
  # };

  # Tests
  enterTest = ''
    echo "Testing Java development environment..."
    java -version
    gradle --version
    ./gradlew check
  '';
}
