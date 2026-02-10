#!/usr/bin/env node
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

/**
 * Generate a minimal valid ICO file (1x1 pixel, 32-bit RGBA)
 * ICO format reference: https://en.wikipedia.org/wiki/ICO_(file_format)
 */

function generateIco() {
  const iconDir = path.join(__dirname, 'src-tauri', 'icons');
  
  // Create icons directory if it doesn't exist
  if (!fs.existsSync(iconDir)) {
    fs.mkdirSync(iconDir, { recursive: true });
  }

  // Minimal 1x1 32-bit RGBA pixel (white)
  const bmpData = Buffer.alloc(1084); // BMP header (40 bytes) + 1 pixel (4 bytes) + padding

  // BMP Header (40 bytes after ICONDIR entry)
  let offset = 0;
  
  // BITMAPINFOHEADER (40 bytes)
  bmpData.writeUInt32LE(40, offset); offset += 4;        // Header size
  bmpData.writeInt32LE(1, offset); offset += 4;          // Width: 1 pixel
  bmpData.writeInt32LE(2, offset); offset += 4;          // Height: 2 pixels (1 for image, 1 for AND mask)
  bmpData.writeUInt16LE(1, offset); offset += 2;         // Color planes
  bmpData.writeUInt16LE(32, offset); offset += 2;        // Bits per pixel (32-bit RGBA)
  bmpData.writeUInt32LE(0, offset); offset += 4;         // Compression (none)
  bmpData.writeUInt32LE(8, offset); offset += 4;         // Image size
  bmpData.writeInt32LE(0, offset); offset += 4;          // X pixels per meter
  bmpData.writeInt32LE(0, offset); offset += 4;          // Y pixels per meter
  bmpData.writeUInt32LE(0, offset); offset += 4;         // Colors used
  bmpData.writeUInt32LE(0, offset); offset += 4;         // Important colors

  // RGBA pixel data (1 pixel: white with full opacity)
  bmpData.writeUInt8(255, offset);     // B (blue)
  bmpData.writeUInt8(255, offset + 1); // G (green)
  bmpData.writeUInt8(255, offset + 2); // R (red)
  bmpData.writeUInt8(255, offset + 3); // A (alpha)

  // Build ICO file
  const icoBuffer = Buffer.alloc(6 + 16 + bmpData.length);
  let icoOffset = 0;

  // ICONDIR header (6 bytes)
  icoBuffer.writeUInt16LE(0, icoOffset); icoOffset += 2; // Reserved (0)
  icoBuffer.writeUInt16LE(1, icoOffset); icoOffset += 2; // Type (1 = icon)
  icoBuffer.writeUInt16LE(1, icoOffset); icoOffset += 2; // Number of images (1)

  // ICONDIRENTRY (16 bytes per image)
  icoBuffer.writeUInt8(1, icoOffset); icoOffset++;       // Width (1 pixel, 0 means 256)
  icoBuffer.writeUInt8(1, icoOffset); icoOffset++;       // Height (1 pixel, 0 means 256)
  icoBuffer.writeUInt8(0, icoOffset); icoOffset++;       // Color count (0 = not used)
  icoBuffer.writeUInt8(0, icoOffset); icoOffset++;       // Reserved (0)
  icoBuffer.writeUInt16LE(1, icoOffset); icoOffset += 2; // Color planes (1)
  icoBuffer.writeUInt16LE(32, icoOffset); icoOffset += 2; // Bits per pixel (32)
  icoBuffer.writeUInt32LE(bmpData.length, icoOffset); icoOffset += 4; // Size of image data
  icoBuffer.writeUInt32LE(22, icoOffset); icoOffset += 4; // Offset to image data (after ICONDIR + ICONDIRENTRY)

  // Copy BMP data
  bmpData.copy(icoBuffer, icoOffset);

  // Write ICO file
  const icoPath = path.join(iconDir, 'icon.ico');
  fs.writeFileSync(icoPath, icoBuffer);
  console.log(`✓ Generated valid ICO file: ${icoPath}`);

  // Also create PNG for other purposes
  const pngPath = path.join(iconDir, 'icon.png');
  const pngBase64 = 'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==';
  fs.writeFileSync(pngPath, Buffer.from(pngBase64, 'base64'));
  console.log(`✓ Generated PNG file: ${pngPath}`);
}

try {
  generateIco();
  console.log('✓ Icon generation completed successfully');
  process.exit(0);
} catch (error) {
  console.error('✗ Icon generation failed:', error.message);
  process.exit(1);
}
