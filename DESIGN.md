**Provides libraries to interface with SAFE network via terminal or headless system, abstracting away necessary state and IPC concerns while allowing the flexibility of redirecting and piping with standard terminal commands.**

This summary may appear as the obvious expectation for a CLI binary, but it's meant to point out the contrast to the initial [implementation](https://github.com/hunterlester/safe_cli/tree/2475d66fc5452a023d858fe400b137098d2f88d5) which keeps the user tied to a CLI process, failing to provide the flexibility to interprocess with other commands. Additionally, user is disconnected from network plus loses App state upon exiting CLI process.

---
Primary components:
##### File Descriptor Library
*In this document you'll see the term `file descriptor` which is meant to be a generic cross-platform term for differing platform IPC implementations*
- Handles single instance management and IPC
- Following Chromium's example, on Windows IPC and single instance management will be handled with [named mutex](https://github.com/electron/electron/blob/master/chromium_src/chrome/browser/process_singleton_win.cc#L273)
- Following Chromium's example, on Linux IPC and single instance management will be handled with [Unix domain socket](https://github.com/electron/electron/blob/master/chromium_src/chrome/browser/process_singleton_posix.cc#L5) 

##### Authenticator
- Single instance per operating system
- polls unique file descriptor for data writes
- holds Authenticator instance in memory

##### CLI Application 
- One unique instance per application
- polls unique file descriptor for data writes
- holds App instance handle in memory

#### SAFE CLI binary
- global command binary to interface with authenticator process and CLI application processes 

---
#### Example flow:
```bash
$ safenetwork create_acc <locator> <password> <invite>
``` 
Initiates program to create secure credentials on network. On completion, if `Authenticator` process running, credentials sent via file descriptor to `Authenticator` process, where `create_acc` is called and `Authenticator` struct is held in memory.  
If `Authenticator` process isn't running, `Authenticator` process started with input credentials.

```
$ safenetwork login <locator> <password>
```
User inputs credentials, which are sent via file descriptor to `Authenticator` process, where `login` is called and [Authenticator](https://github.com/maidsafe/safe_client_libs/blob/master/safe_authenticator/src/lib.rs#L115) struct is held in memory.  
If `Authenticator` process isn't running, `Authenticator` process started with input credentials.

```
$ safenetwork initialise <app_info and permissions as JSON>
```
Spawns process for `CLI Application`, generates `AppExchangeInfo` struct based on parsed JSON input, to be held in memory.

```
$ safenetowrk authorise <app_info.id> <permissions as JSON>
```
Request sent to file descriptor, based on app info id, associated with `CLI Application` which then encodes auth request and sends to `Authenticator` via `Authenticator`-associated file descriptor.after approval `CLI Application` process will receive [AuthGranted](https://github.com/maidsafe/safe_client_libs/blob/master/safe_core/src/ipc/resp.rs#L64) struct

```
$ safenetwork registered <app_info.id>
```
Request sent to `CLI Application` process via associated file descriptor based on app info ID. A registered session is created, connecting Authorised application to network, returning an App struct which maintains connection and allows data manipulations to be made on the network. [App](https://github.com/maidsafe/safe_client_libs/blob/master/safe_app/src/lib.rs#L128) struct held in memory in associated `CLI Application` process, which is listening for further SAFE network data operations.

```
$ safenetwork quick_app
```
As a convenience to human users, default `CLI Application` is initialised, authorised with default permissions, and registered on network.

```
$ safenetwork upload <file path> <app_info.id>
```
File path string is sent via file descriptor to `CLI Application` process, where file buffer is retrieved and uploaded to network.  
If no app ID is supplied, file is uploaded via default `CLI application` created with `safenetwork quick_app`
