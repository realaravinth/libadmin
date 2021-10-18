<div align="center"><h1>libadmin</h1>

[![Documentation](https://img.shields.io/badge/docs-master-blue)](https://realaravinth.github.io/libadmin/)
[![Build](https://github.com/realaravinth/libadmin/actions/workflows/linux.yml/badge.svg)](https://github.com/realaravinth/libadmin/actions/workflows/linux.yml)
[![codecov](https://codecov.io/gh/realaravinth/libadmin/branch/master/graph/badge.svg?token=TYZXLOOHYQ)](https://codecov.io/gh/realaravinth/libadmin)

[![dependency status](https://deps.rs/repo/github/realaravinth/libadmin/status.svg)](https://deps.rs/repo/github/realaravinth/libadmin)

  <p>
    <strong>Access control, admin panel and access control for web
	servers written in Rust</strong>
  </p>
<br /></div>

The goal is to create something similar to Django admin/PHP MyAdmin,
complete with access control mechanism and web forms.

## Code Organisation

### Database

| crate                                             | documentation                                                                                                                 | description                                                                                                  |
| ------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| [`db-core`](./database/db-core)                   | [![](https://img.shields.io/badge/docs-db--core-orange)](https://realaravinth.github.io/libadmin/db_core/)                    | Collection of traits describing all `libadmin` database operations. Used to implement support a new database |
| [`db-sqlx-postgres`](./database/db-sqlx-postgres) | [![](https://img.shields.io/badge/docs-db--sqlx--postgres-orange)](https://realaravinth.github.io/libadmin/db_sqlx_postgres/) | Database support for postgres using [`sqlx`](https://crates.io/crates/sqlx)                                  |

### Web frameworks

| crate                                          | documentation                                                                               | description                          |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------- | ------------------------------------ |
| [`libadmin-actix-web`](./framework/actix-web/) | [![](https://img.shields.io/badge/docs-libadmin--actix--web-green)](./framework/actix-web/) | `libadmin` implemented for actix-web |

## Support for database `foo` and framework `bar`

I've tried to be very general with the implementation and I've provided
mechanisms to hook unsupported databases and web frameworks. For now, I plan
on implementing support for only for the libraries that I'm familiar
with.

However, if you decide to implement support for something, feel
free to contact me. I'd be happy to help and I'll link your work on
here :)
