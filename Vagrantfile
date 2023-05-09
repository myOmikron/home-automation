Vagrant.configure("2") do |config|
  config.nfs.functional = false
  config.vm.synced_folder "./", "/vagrant", type: "virtiofs"

  config.vm.define "brain", primary: true do |brain|
    brain.vm.hostname = "brain"
    brain.vm.box = "generic/debian11"
    brain.vm.network "forwarded_port", guest: 443, host: 8443
    brain.vm.network "forwarded_port", guest: 8883, host: 8883
    brain.vm.network :private_network, :ip => '10.45.45.10'
    brain.vm.provider "libvirt" do |vb|
        vb.memory = "512"
        vb.cpus = "2"
        vb.memorybacking :access, :mode => "shared"
    end
    brain.vm.provision :ansible do |a|
      a.playbook = "vagrant/brain.yml"
    end
  end
end