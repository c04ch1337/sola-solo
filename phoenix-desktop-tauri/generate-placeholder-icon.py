#!/usr/bin/env python3
"""
Generate a placeholder 1024x1024 PNG icon for Sola AGI.
Creates a simple flame/phoenix-themed icon with gradient background.
"""

try:
    from PIL import Image, ImageDraw, ImageFont
except ImportError:
    print("Error: Pillow not installed. Install with: pip install Pillow")
    exit(1)

import os
import sys

def generate_icon(output_path: str):
    """Generate a 1024x1024 placeholder icon with Sola AGI branding."""
    
    # Create 1024x1024 image with transparency
    size = 1024
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Background gradient (dark purple to dark blue)
    for y in range(size):
        # Gradient from #1a1a2e to #16213e
        r = int(26 + (22 - 26) * (y / size))
        g = int(26 + (33 - 26) * (y / size))
        b = int(46 + (62 - 46) * (y / size))
        draw.rectangle([(0, y), (size, y + 1)], fill=(r, g, b, 255))
    
    # Draw central orb (AI consciousness)
    center = size // 2
    orb_size = size // 3
    orb_outer = orb_size + 20
    
    # Outer glow (cyan)
    draw.ellipse(
        [center - orb_outer, center - orb_outer, center + orb_outer, center + orb_outer],
        fill=(0, 255, 255, 60)  # Cyan with transparency
    )
    
    # Orb gradient (orange to yellow - phoenix fire)
    for i in range(orb_size, 0, -5):
        alpha = int(255 * (1 - i / orb_size))
        color = (
            int(255 - (255 - 255) * (i / orb_size)),  # R: 255
            int(200 - (200 - 200) * (i / orb_size)),  # G: 200
            int(100 - (100 - 100) * (i / orb_size)),  # B: 100
            alpha
        )
        draw.ellipse(
            [center - i, center - i, center + i, center + i],
            fill=color
        )
    
    # Draw stylized "S" shape (phoenix wings)
    # Simplified S-curve representing Sola
    points = []
    for x in range(center - orb_size, center + orb_size):
        y_offset = int(50 * (x - center) / orb_size)
        y1 = center - orb_size // 2 + y_offset
        y2 = center + orb_size // 2 + y_offset
        points.append((x, y1))
        points.append((x, y2))
    
    # Draw orbital rings
    for ring_radius in [orb_size + 40, orb_size + 80]:
        draw.ellipse(
            [center - ring_radius, center - ring_radius, center + ring_radius, center + ring_radius],
            outline=(0, 255, 255, 100),
            width=3
        )
    
    # Add text "S" in center (if font available)
    try:
        # Try to use a system font
        font_size = size // 4
        try:
            font = ImageFont.truetype("arial.ttf", font_size)
        except:
            try:
                font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", font_size)
            except:
                font = ImageFont.load_default()
        
        # Draw "S" text
        text = "S"
        bbox = draw.textbbox((0, 0), text, font=font)
        text_width = bbox[2] - bbox[0]
        text_height = bbox[3] - bbox[1]
        text_x = center - text_width // 2
        text_y = center - text_height // 2
        
        # Draw text with glow effect
        for offset in [(0, 0), (-2, -2), (2, 2), (-2, 2), (2, -2)]:
            draw.text(
                (text_x + offset[0], text_y + offset[1]),
                text,
                font=font,
                fill=(255, 255, 255, 200 if offset == (0, 0) else 50)
            )
    except Exception as e:
        # If font fails, just draw a simple circle
        pass
    
    # Save the image
    img.save(output_path, 'PNG')
    print(f"✅ Generated placeholder icon: {output_path}")
    print(f"   Size: {size}x{size} pixels")
    print(f"   Format: PNG (RGBA)")
    return True

if __name__ == '__main__':
    # Determine output path
    script_dir = os.path.dirname(os.path.abspath(__file__))
    output_path = os.path.join(script_dir, 'src-tauri', 'icons', 'icon.png')
    
    # Create icons directory if it doesn't exist
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    
    # Check if icon already exists
    if os.path.exists(output_path):
        response = input(f"Icon already exists at {output_path}. Overwrite? (y/N): ")
        if response.lower() != 'y':
            print("Skipping icon generation.")
            sys.exit(0)
    
    # Generate icon
    if generate_icon(output_path):
        print("\n📦 Next steps:")
        print("   1. Review the generated icon.png")
        print("   2. Run: cargo tauri icon src-tauri/icons/icon.png")
        print("   3. Or run: npm run icon (if package.json script exists)")
    else:
        print("❌ Failed to generate icon.")
        sys.exit(1)
