## Core Rules

### DRY (Don't Repeat Yourself)
- Zero code duplication
- Each functionality exists in exactly one place
- No duplicate files or alternative implementations

### KISS (Keep It Simple, Stupid)
- Implement the simplest solution that works
- No over-engineering
- Use existing systems, don't recreate them

### Transparent Error Handling
- No error hiding or fallback mechanisms
- All errors properly displayed
- Fix root causes, don't mask them

### FORBIDDEN PATTERNS
- **NO** keeping old and new code together
- **NO** simplified implementations when real ones exist
- **NO** migration functions or compatibility layers
- **NO** versioned function names (processV2, handleNew)
- **NO** TODOs in production code
