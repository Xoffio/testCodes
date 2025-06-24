# Terraform Telmate Proxmox provider

This is a simple example to terraform Proxmox using the telmate provider

To be able to use this you will need:
- Create a role, user, and a token in Proxmox for Terraform
- Have a Cloud-Init ready template

Once you have that, open the `terraform.tfvars` file and fill the next variables:

```toml
pm_api_url          = "https://proxmox-url.lan:8006/api2/json" # Proxmox URL
pm_api_token_id     = "terraform-prov@pve!terraform-token" # Proxmox token's ID
pm_api_token_secret = "TOKEN-SECRET" # Proxmox token's secret
vm_name             = "terra-vm" # Name for the new VM
disk_size           = 32 # Disk size in GB
ram_size            = 2048 # RAM size in MB
target_node         = "prox-machine01" # Name of the Proxmox server to create the VM in.
template_name       = "ubuntu-cloud-2404" # Name of the Cloud-Init ready template
storage_name        = "local-lvm" # Storage to save the disk to
vm_ip               = "192.168.1.222" # IP for the new VM
gateway_ip          = "192.168.1.1" # Gateway of the network
dns_ip              = "192.168.1.15" # DNS's IP
vm_user             = "ubuntu" # Default username
ssh_key             = "ssh-rsa KEY USER@HOST" # SSH key to inject in the VM
```

Once you fill down the variables you can run:

```bash
# Initialize the Terraform working directory.
# This downloads required provider plugins, sets up the backend, and prepares the environment.
terraform init

# Create an execution plan.
# This shows what actions Terraform will take without making any changes.
# Useful for reviewing what will be created, modified, or destroyed.
terraform plan

# Apply the changes required to reach the desired state.
# This executes the actions from the plan and provisions real infrastructure.
terraform apply
```
