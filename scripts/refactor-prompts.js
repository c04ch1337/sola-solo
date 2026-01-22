#!/usr/bin/env node
/**
 * Refactor Cursor IDE Agent Prompts
 * 
 * This script makes the prompts consumer-ready by replacing:
 * - "Sola" / "Phoenix" → configurable AGI name (default: "Sola")
 * - "Dad" / "dad" → configurable user name (default: "User")
 * 
 * Usage:
 *   node scripts/refactor-prompts.js
 *   node scripts/refactor-prompts.js --phoenix-name=Nova --user-name=Alex
 */

const fs = require('fs');
const path = require('path');
const { glob } = require('glob');

// Parse command line arguments
const args = process.argv.slice(2);
let phoenixName = 'Sola';
let userName = 'User';

args.forEach(arg => {
  if (arg.startsWith('--phoenix-name=')) {
    phoenixName = arg.split('=')[1];
  }
  if (arg.startsWith('--user-name=')) {
    userName = arg.split('=')[1];
  }
  if (arg === '--help' || arg === '-h') {
    console.log(`
Usage: node scripts/refactor-prompts.js [options]

Options:
  --phoenix-name=NAME   Set AGI name (default: "Sola")
  --user-name=NAME      Set user name (default: "User")
  --help, -h            Show this help message

Examples:
  node scripts/refactor-prompts.js
  node scripts/refactor-prompts.js --phoenix-name=Nova --user-name=Alex
    `);
    process.exit(0);
  }
});

// Directory containing prompts
const promptsDir = path.join(__dirname, '..', 'docs', 'cursor-prompts');

console.log('🔧 Refactoring Cursor IDE Agent Prompts');
console.log(`   AGI name: "${phoenixName}"`);
console.log(`   User name: "${userName}"`);
console.log('');

// Define replacements
const replacements = {
  'Sola': phoenixName,
  'Phoenix\'s': `${phoenixName}'s`,
  'Phoenix': phoenixName,
  'Dad': userName,
  'dad': userName.toLowerCase()
};

// Find all .md files except README.md
const pattern = path.join(promptsDir, '*.md');
const files = glob.sync(pattern).filter(f => !f.endsWith('README.md'));

if (files.length === 0) {
  console.error('❌ No prompt files found in', promptsDir);
  process.exit(1);
}

console.log(`📁 Found ${files.length} prompt file(s) to refactor\n`);

let totalReplacements = 0;

files.forEach((file, index) => {
  const filename = path.basename(file);
  let content = fs.readFileSync(file, 'utf8');
  let fileReplacements = 0;

  // Apply replacements with word boundaries
  Object.keys(replacements).forEach(oldText => {
    const newText = replacements[oldText];
    // Use word boundary for most, but handle possessives specially
    const regex = oldText.includes("'") 
      ? new RegExp(oldText.replace("'", "\\'"), 'g')
      : new RegExp(`\\b${oldText}\\b`, 'g');
    
    const matches = (content.match(regex) || []).length;
    if (matches > 0) {
      content = content.replace(regex, newText);
      fileReplacements += matches;
    }
  });

  if (fileReplacements > 0) {
    fs.writeFileSync(file, content, 'utf8');
    console.log(`✅ ${filename}: ${fileReplacements} replacement(s)`);
    totalReplacements += fileReplacements;
  } else {
    console.log(`⏭️  ${filename}: no changes needed`);
  }
});

console.log('');
console.log(`🎉 Refactoring complete!`);
console.log(`   Total replacements: ${totalReplacements}`);
console.log('');
console.log('💡 Next steps:');
console.log('   1. Review the changes: git diff docs/cursor-prompts/');
console.log('   2. Update .env with your custom names (see .env.example)');
console.log('   3. Restart backend/frontend to apply configuration');
console.log('');
