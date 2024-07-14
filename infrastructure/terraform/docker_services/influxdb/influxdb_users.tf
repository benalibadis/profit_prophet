locals {
  influxdb_users = nonsensitive([
    for idx, user in var.influxdb.users : {
      idx        = idx
      is_admin   = user.is_admin
      grants     = user.grants
    }
  ])
}

resource "influxdb_user" "user" {
  for_each = { for user in local.influxdb_users : user.idx => user }

  name     = var.influxdb.users[tonumber(each.key)].username
  password = var.influxdb.users[tonumber(each.key)].password
  admin    = each.value.is_admin

  grant {
    database = "profit_prophet"
    privilege = "ALL"
  }

  depends_on = [ influxdb_database.databases ]
}