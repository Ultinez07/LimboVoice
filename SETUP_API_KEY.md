# Setting Up Limbo Voice with OpenAI Whisper

## 🔑 Required: OpenAI API Key

Limbo Voice uses **OpenAI Whisper Turbo** for fast, accurate speech-to-text transcription.

### Step 1: Get Your API Key

1. Go to https://platform.openai.com/api-keys
2. Create a new API key
3. Copy it (starts with `sk-...`)

### Step 2: Set Environment Variable

#### Windows (Permanent):
```powershell
# Run PowerShell as Administrator
[System.Environment]::SetEnvironmentVariable('OPENAI_API_KEY', 'sk-your-key-here', 'User')
```

#### Windows (Temporary - for testing):
```powershell
$env:OPENAI_API_KEY="sk-your-key-here"
# Then launch Limbo Voice from this PowerShell window
```

### Step 3: Restart Limbo Voice

Close and reopen Limbo Voice for the environment variable to take effect.

## 💰 Costs

- **Whisper Turbo**: $0.006 per minute of audio
- **Example**: 100 recordings (10 seconds each) = ~16 minutes = **$0.10**
- Monthly usage (heavy): **$2-5**

## ✅ Testing

1. Launch Limbo Voice
2. Press **Alt+Space**
3. Speak: "Hello world"
4. Release **Alt+Space**
5. Text should type automatically!

## ⚠️ Troubleshooting

### Error: "OPENAI_API_KEY not set"
- Make sure you set the environment variable correctly
- Restart your computer if needed
- Try the temporary method first to test

### Error: "API error: 401"
- Your API key is invalid or expired
- Generate a new key from OpenAI dashboard

### No text typed
- Check your internet connection
- Make sure you spoke clearly for at least 1-2 seconds
- Check the overlay for error messages
