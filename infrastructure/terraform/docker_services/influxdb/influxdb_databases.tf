resource "influxdb_database" "databases" {
    for_each = { for idx, db in nonsensitive(var.influxdb.databases) : idx => db }
    name = each.value.name
}