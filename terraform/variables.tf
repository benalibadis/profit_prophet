# Authentification

variable "api_authentification" {

    type = object({
        tenancy_ocid        = string
        user_ocid           = string
        fingerprint         = string
        private_key         = string
        region              = string
    })

    sensitive = true
}