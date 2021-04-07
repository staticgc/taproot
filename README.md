# taproot

Taproot is a collection of  projects.


* **vstore** => Start versioning any key-value pair database
* **tapfs** => FileSystem developed on top of vstore

The filesystem there supports following features:

1. Versioning
2. Ability to separate FileSystem metadata (namespace, extent map) and data
3. Allow git like syncing of the fs metadata
4. Ability to diff between any two snapshots including offset level changes to files
5. Ability to partially sync snapshots (almost equivalent to `git clone --depth=<n>` option)


## Databases Supported at present

1. Sqlite DB (not technically a KV database but used as one)
2. Rocks DB


## Intended use-cases as of now

1. Backup
2. Dropbox like Personal data sync to remote location (e.g cloud)
3. SqliteDB-on-S3. The fs (tapfs) can be a VFS for sqlite DB which allows to run a RDMBS on S3. This can be useful in AWS lambda 
