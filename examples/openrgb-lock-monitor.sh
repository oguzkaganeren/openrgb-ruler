#!/bin/bash
# ============================================================
# OpenRGB - KDE Lock/Unlock LED Kontrolü
# CachyOS KDE için
# Kullanım: ~/.config/autostart/ içine servis olarak ekle
# ============================================================

OPENRGB_BIN="${OPENRGB_BIN:-openrgb}"

# --- LED Profilleri ---
# Kilitleme anında LED rengi (kırmızı, parlaklık 0 = KAPALI)
lock_action() {
    # Tüm cihazları kapat
    "$OPENRGB_BIN" --noautoconnect -c 000000 2>/dev/null
    # VEYA belirli bir profil yükle:
    # "$OPENRGB_BIN" --noautoconnect -p "locked" 2>/dev/null
}

# Kilit açılınca LED'leri aç
unlock_action() {
    # Beyaz renk ile aç (istediğin renge göre değiştir)
    "$OPENRGB_BIN" --noautoconnect -c 8F12CE 2>/dev/null
    # VEYA profil yükle:
    # "$OPENRGB_BIN" --noautoconnect -p "unlocked" 2>/dev/null
}

# ============================================================
# DBus üzerinden KDE Screensaver sinyalini dinle
# ScreenSaverActive = true  → kilitlendi
# ScreenSaverActive = false → açıldı
# ============================================================

echo "[openrgb-lock-monitor] Başlatıldı. DBus dinleniyor..."

dbus-monitor --session \
    "type='signal',interface='org.freedesktop.ScreenSaver'" \
    "type='signal',interface='com.canonical.Unity.Session'" \
    "type='signal',member='ActiveChanged'" 2>/dev/null | \
while read -r line; do
    # KDE Screensaver: ActiveChanged true/false
    if echo "$line" | grep -q "ActiveChanged"; then
        read -r next_line
        if echo "$next_line" | grep -q "true"; then
            echo "[openrgb-lock-monitor] 🔒 Ekran kilitledi → LED KAPANIYOR"
            lock_action
        elif echo "$next_line" | grep -q "false"; then
            echo "[openrgb-lock-monitor] 🔓 Ekran açıldı  → LED AÇILIYOR"
            unlock_action
        fi
    fi

    # org.freedesktop.login1 Lock/Unlock sinyalleri (fallback)
    if echo "$line" | grep -q "member=Lock"; then
        echo "[openrgb-lock-monitor] 🔒 loginctl Lock → LED KAPANIYOR"
        lock_action
    elif echo "$line" | grep -q "member=Unlock"; then
        echo "[openrgb-lock-monitor] 🔓 loginctl Unlock → LED AÇILIYOR"
        unlock_action
    fi
done
