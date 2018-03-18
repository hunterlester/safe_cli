##### Authenticator
- creates separate process, creates named pipe, polls file descriptor for data writes
- holds Authenticator instance in memory

##### SAFE CLI APPLICATION
- creates separate process, creates named pipe, polls file descriptor for data writes
- upon execution, connects to named pipe associated with Authenticator, initialises, authorises, registers on network
- holds App instance handle in memory

#### SAFE CLI binary
- commands to interface with authenticator process and CLI application process depending on need

---
- SAFE CLI binary commands can be run anywhere on system to interface with network
- For example, one may want to simply `cd` into any directory on system to quickly upload files to network
Flow example:
- binary `create_acc` -> initiates cli program to create secure credentials. On completion, if Authenticator process running, credentials sent via named pipe to Authenticator process, where `create_acc` is called and `Authenticator` struct is held in memory, then executes SAFE CLI Application.
 If Authenticator process isn't running, Authenticator process started with input credentials.

- binary login -> user inputs credentials, which are sent via named pipe to Authenticator process, where `login` is called and `Authenticator` struct is held in memory, then executes SAFE CLI Application
 If Authenticator process isn't running, Authenticator process started with input credentials.

- binary upload -> user inputs file path, which is sent via named pipe to SAFE CLI Application, where file buffer is retrieved and uploaded to network.
