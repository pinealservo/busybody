# Busybody

A framework for generating potentially interesting system activity

## Supported Activity Types

+ Process Creation
+ File Creation, Modification, Deletion
+ TCP/IP Connection + Transmission

## Logged Information

For each activity, busybody records a log entry. These are JSON objects with the
following fields:

### Common Fields

+ Event Timestamp
+ Event Type
+ Username of the controlling user
+ Process Record
  - Process Name
  - Process ID
  - Process Command Line

### File Operation Specific Fields

+ Full path name to the file operated on
+ Operation Type (Create, Modify, Delete)

### Network Operation Specific Fields

+ Destination IP Address and Port
+ Source IP Address and Port
+ Protocol Name
+ Byte count of transmitted data
