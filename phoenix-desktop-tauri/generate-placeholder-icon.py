#!/usr/bin/env python3
"""
Sola AGI - Placeholder Icon Generator
Creates a 1024x1024 PNG icon with flame/circle design
"""

try:
    from PIL import Image, ImageDraw, ImageFont
    import os
except ImportError:
    print("❌ Error: Pillow (PIL) not installed")
    print("Install with: pip install Pillow")
    exit(1)

def create_sola_icon(output_path="src-tauri/icons/icon.png"):
    """Create a beautiful Sola AGI icon with flame design"""
    
    # Create 1024x1024 transparent image
    size = 1024
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Color scheme
    purple = '#6B46C1'    # Primary - wisdom, spirituality
    orange = '#FF6B35'    # Accent - energy, warmth
    yellow = '#FFD23F'    # Highlight - light, hope
    white = '#FFFFFF'     # Core - purity
    
    # Draw layered circles (flame effect)
    center = size // 2
    
    # Outer glow (purple)
    draw.ellipse([50, 50, size-50, size-50], fill=purple)
    
    # Middle layer (orange)
    draw.ellipse([150, 150, size-150, size-150], fill=orange)
    
    # Inner layer (yellow)
    draw.ellipse([250, 250, size-250, size-250], fill=yellow)
    
    # Core (white)
    draw.ellipse([350, 350, size-350, size-350], fill=white)
    
    # Add "S" letter in center
    try:
        # Try to load a nice font
        font_size = 400
        font = ImageFont.truetype("arial.ttf", font_size)
    except:
        try:
            font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 400)
        except:
            try:
                font = ImageFont.truetype("C:\\Windows\\Fonts\\arial.ttf", 400)
            except:
                print("⚠️  Warning: Could not load custom font, using default")
                font = None
    
    if font:
        # Draw "S" in purple
        bbox = draw.textbbox((0, 0), "S", font=font)
        text_width = bbox[2] - bbox[0]
        text_height = bbox[3] - bbox[1]
        x = (size - text_width) // 2 - bbox[0]
        y = (size - text_height) // 2 - bbox[1]
        draw.text((x, y), "S", fill=purple, font=font)
    
    # Ensure output directory exists
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    
    # Save icon
    img.save(output_path, 'PNG')
    print(f"✅ Created icon: {output_path}")
    print(f"   Size: {size}x{size} PNG")
    print(f"   Colors: Purple, Orange, Yellow, White")
    
    return output_path

def create_simple_icon(output_path="src-tauri/icons/icon.png"):
    """Create a simpler icon with just 'S' on colored background"""
    
    size = 1024
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Draw purple circle background
    draw.ellipse([50, 50, size-50, size-50], fill='#6B46C1')
    
    # Try to add "S" letter
    try:
        font = ImageFont.truetype("arial.ttf", 600)
    except:
        try:
            font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 600)
        except:
            try:
                font = ImageFont.truetype("C:\\Windows\\Fonts\\arial.ttf", 600)
            except:
                font = None
    
    if font:
        bbox = draw.textbbox((0, 0), "S", font=font)
        text_width = bbox[2] - bbox[0]
        text_height = bbox[3] - bbox[1]
        x = (size - text_width) // 2 - bbox[0]
        y = (size - text_height) // 2 - bbox[1]
        draw.text((x, y), "S", fill='white', font=font)
    
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    img.save(output_path, 'PNG')
    print(f"✅ Created simple icon: {output_path}")
    
    return output_path

if __name__ == "__main__":
    import sys
    
    print("🎨 Sola AGI Icon Generator")
    print("=" * 50)
    print()
    
    # Check if icons directory exists
    icons_dir = "src-tauri/icons"
    if not os.path.exists(icons_dir):
        os.makedirs(icons_dir)
        print(f"📁 Created directory: {icons_dir}")
    
    # Check for existing icon
    icon_path = os.path.join(icons_dir, "icon.png")
    if os.path.exists(icon_path):
        response = input(f"⚠️  {icon_path} already exists. Overwrite? (y/N): ")
        if response.lower() != 'y':
            print("❌ Cancelled")
            sys.exit(0)
    
    # Ask which style
    print()
    print("Choose icon style:")
    print("1. Flame design (layered circles)")
    print("2. Simple 'S' on purple circle")
    print()
    choice = input("Enter choice (1 or 2, default=1): ").strip()
    
    print()
    if choice == "2":
        create_simple_icon(icon_path)
    else:
        create_sola_icon(icon_path)
    
    print()
    print("📋 Next steps:")
    print("1. Review icon: open src-tauri/icons/icon.png")
    print("2. Generate all formats: cargo tauri icon src-tauri/icons/icon.png")
    print("3. Rebuild app: npm run build")
    print()
    print("🕊️ Icon generation complete!")
