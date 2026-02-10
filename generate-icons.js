#!/usr/bin/env node
/**
 * Generate placeholder icons for Tauri development
 * This creates minimal valid PNG files that Tauri can use
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const iconDir = path.join(__dirname, 'src-tauri', 'icons');

// Ensure icons directory exists
if (!fs.existsSync(iconDir)) {
  fs.mkdirSync(iconDir, { recursive: true });
}

// Minimal 1x1 PNG (base64 - light blue background)
const minimalPng = Buffer.from(
  'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==',
  'base64'
);

// Create all required icon sizes
const sizes = ['32x32', '128x128', '256x256', '512x512'];
for (const size of sizes) {
  fs.writeFileSync(path.join(iconDir, `${size}.png`), minimalPng);
  fs.writeFileSync(path.join(iconDir, `${size}@2x.png`), minimalPng);
}

// Create ICO file (Windows) - use minimal PNG data
fs.writeFileSync(path.join(iconDir, 'icon.ico'), minimalPng);
fs.writeFileSync(path.join(iconDir, 'icon.png'), minimalPng);

console.log('✓ Placeholder icons generated successfully');

