#!/usr/bin/env node
/**
 * Sola AGI Icon Converter
 * Converts SVG to PNG using sharp (Node.js)
 * 
 * Usage: node convert-icon.js
 */

const fs = require('fs');
const path = require('path');

async function convertIcon() {
    console.log('🕊️ Sola AGI Icon Converter');
    console.log('=========================');
    
    const svgPath = path.join(__dirname, 'src-tauri', 'icons', 'icon.svg');
    const pngPath = path.join(__dirname, 'src-tauri', 'icons', 'icon.png');
    
    // Check if SVG exists
    if (!fs.existsSync(svgPath)) {
        console.error('❌ Error: icon.svg not found at', svgPath);
        process.exit(1);
    }
    
    console.log('📄 Found SVG:', svgPath);
    
    // Try to use sharp
    try {
        const sharp = require('sharp');
        
        console.log('📐 Converting SVG to PNG using sharp...');
        
        await sharp(svgPath)
            .resize(1024, 1024)
            .png()
            .toFile(pngPath);
        
        console.log('✅ Created icon.png (1024x1024)');
        console.log('📍 Output:', pngPath);
        console.log('');
        console.log('Next step: Run "cargo tauri icon src-tauri/icons/icon.png"');
        
    } catch (err) {
        if (err.code === 'MODULE_NOT_FOUND') {
            console.log('⚠️ sharp not installed. Installing...');
            
            const { execSync } = require('child_process');
            try {
                execSync('npm install sharp', { stdio: 'inherit', cwd: __dirname });
                console.log('✅ sharp installed. Please run this script again.');
            } catch (installErr) {
                console.error('❌ Failed to install sharp:', installErr.message);
                console.log('');
                console.log('Manual installation:');
                console.log('  cd phoenix-desktop-tauri');
                console.log('  npm install sharp');
                console.log('  node convert-icon.js');
            }
        } else {
            console.error('❌ Error converting icon:', err.message);
        }
        process.exit(1);
    }
}

convertIcon();
