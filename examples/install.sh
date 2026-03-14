#!/bin/bash
# OpenRGB Lock Monitor - Kurulum Scripti

set -e
echo "=== OpenRGB Lock Monitor Kurulumu ==="

# 1. Script'i kopyala
mkdir -p ~/.local/bin
cp openrgb-lock-monitor.sh ~/.local/bin/openrgb-lock-monitor.sh
chmod +x ~/.local/bin/openrgb-lock-monitor.sh
echo "✓ Script: ~/.local/bin/openrgb-lock-monitor.sh"

# 2. Systemd user service kur
mkdir -p ~/.config/systemd/user
cp openrgb-lock-monitor.service ~/.config/systemd/user/
echo "✓ Service: ~/.config/systemd/user/openrgb-lock-monitor.service"

# 3. Servisi aktifleştir
systemctl --user daemon-reload
systemctl --user enable --now openrgb-lock-monitor.service
echo "✓ Servis aktifleştirildi ve başlatıldı"

echo ""
echo "=== Kurulum Tamamlandı ==="
echo "Durumu kontrol etmek için:"
echo "  systemctl --user status openrgb-lock-monitor"
echo ""
echo "Logları görmek için:"
echo "  journalctl --user -u openrgb-lock-monitor -f"
echo ""
echo "HATIRLATMA: OpenRGB Server'ın çalışıyor olması gerekiyor!"
echo "  openrgb --server &"
