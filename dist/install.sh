# Create user
sudo useradd --system --home /var/lib/mrpump --shell /usr/sbin/nologin mrpump
sudo install -d -o mrpump -g mrpump /var/lib/mrpump

# Install binary
sudo install -d /etc/mrpump
sudo install -m 0755 ./mrpump /usr/local/bin/mrpump
sudo install ./mrpump.service /etc/systemd/system/mrpump.service
sudo install ./config.yaml /etc/mrpump/config.yaml

# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable mrpump