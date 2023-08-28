# Scooter Skipper

An app to auto-skip non-Taylor versions in Spotify

## Inspiration

https://github.com/Roshy10/taylors-version

https://twitter.com/jamesandbetty/status/1692598762005287167

https://github.com/spotify/spotify.github.io/issues/21

## Files

-   [**Cargo.toml**](./Cargo.toml)

    -   The package manager descriptor file for Rust. This file lists the project details and its dependencies.

-   [**LICENSE**](./LICENSE)

    -   The license file for the project.

-   [**README.md**](./README.md)

    -   This file; provides an overview and documentation for the project.

-   [**notes.txt**](./notes.txt)

    -   Miscellaneous notes related to the project.

-   [**src/main.rs**](./src/main.rs)

    -   The main entry point for the application.

-   [**src/auth/mod.rs**](./src/auth/mod.rs)

    -   Module file for authentication-related functions and structs.

-   [**src/auth/spotify.rs**](./src/auth/spotify.rs)

    -   Handles Spotify-specific authentication logic.

-   [**src/config/mod.rs**](./src/config/mod.rs)

    -   Module file for configuration-related functions and structs.

-   [**src/config/settings.rs**](./src/config/settings.rs)

    -   Manages app-specific settings, possibly including Firebase credentials or other configuration data.

-   [**src/handlers/mod.rs**](./src/handlers/mod.rs)

    -   Module file for route handlers.

-   [**src/handlers/playback.rs**](./src/handlers/playback.rs)

    -   Manages playback-related routes, such as monitoring and intervention logic.

-   [**src/handlers/user.rs**](./src/handlers/user.rs)

    -   Manages user-related routes, such as settings or profile info.

-   [**src/models/mod.rs**](./src/models/mod.rs)

    -   Module file for data structures and models.

-   [**src/models/track.rs**](./src/models/track.rs)

    -   Defines the data structure for tracks or albums.

-   [**src/models/user.rs**](./src/models/user.rs)

    -   Defines the data structure for users, including preferences and tokens.

-   [**src/utils/firebase.rs**](./src/utils/firebase.rs)

    -   Contains functions and logic specific to Firebase interactions, such as reading/writing data.

-   [**src/utils/mod.rs**](./src/utils/mod.rs)

    -   Module file for utility functions and helpers.

-   [**tests/test_main.py**](./tests/test_main.py)

    -   Python script to run tests on the main application.

    ## Bob the Builder (Build Plan)

    Basic Server Setup: Before diving into any specific feature, set up a basic Rust web server. This will give you a foundation to build upon. There are several Rust web frameworks to choose from, such as Rocket, Actix-Web, and Warp. Setting up a basic server will allow you to test routes, see logs, and understand the flow of the application.
    Authentication (Spotify):
    This is a crucial part of your app, as many of the subsequent features depend on it.
    Integrate with Spotify's OAuth flow to authenticate users.
    Handle the callback from Spotify and extract the access token (and refresh token, if applicable).
    Store the token in a safe manner (consider using Firebase for this).
    User Preferences (Firebase):
    Once you have authentication in place, you can allow users to set their preferences (e.g., which albums to block).
    Set up routes and UI components to let users select their blocked albums.
    Store these preferences in Firebase.
    Playback Monitoring:
    Implement the logic to check the currently playing track for the authenticated user.
    Compare the track to the user's blocked list.
    If the track is on the blocked list, send a "skip" command.
    Testing:
    As you develop, write unit tests to ensure individual components of your application are working as expected.
    Once major components are in place, consider integration tests to check the flow of your app.
    Error Handling & Edge Cases:
    Handle scenarios where tokens expire and need to be refreshed.
    Manage cases where a user de-authorizes your app from Spotify.
    Implement graceful error messages and recovery options for users.
    Optimization & Refinement:
    Once the core functionality is in place, look for areas to optimize.
    Consider rate-limiting, caching, and other strategies to minimize API calls and improve performance.
    Documentation & User Guides:
    As you approach completion, enhance your README.md and potentially create user guides or FAQs to help users understand how to use the app and what to expect.
    Deployment & Scaling:
    Consider where and how you'll deploy the app.
    Ensure you've set up logging and monitoring to keep an eye on the app's health and performance.
