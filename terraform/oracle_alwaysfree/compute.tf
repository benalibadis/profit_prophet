data "oci_core_images" "ubuntu" {
  compartment_id = oci_identity_compartment.alwaysfree_compartment.id
  operating_system = var.compute.os
  operating_system_version = var.compute.os_version
  shape = var.compute.shape
}

data "oci_identity_availability_domains" "ads" {
  compartment_id = oci_identity_compartment.alwaysfree_compartment.id
}

resource "oci_core_instance" "alwaysfree_instance" {
  availability_domain = data.oci_identity_availability_domains.ads.availability_domains[0].name
  compartment_id      = oci_identity_compartment.alwaysfree_compartment.id
  shape               = var.compute.shape
  
  shape_config {
    ocpus = var.compute.ocpus
    memory_in_gbs = var.compute.memory_in_gbs
  }

  create_vnic_details {
    subnet_id = oci_core_subnet.alwaysfree_subnet.id
    assign_public_ip = true
  }

  source_details {
    source_type = "image"
    source_id   = data.oci_core_images.ubuntu.images[0].id
  }
}
