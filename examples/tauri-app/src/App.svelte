<script>
  import LoadingButton from './lib/LoadingButton.svelte'
  import {
    createDetectorForAllLanguages,
    createDetectorForLanguages,
    detectLanguage,
    computeLanguageConfidence,
    computeLanguageConfidenceValues
  } from 'tauri-plugin-lingua-api'

  let textInput = $state('Hello, how are you?')
  let selectedLanguages = $state('en,fr,de,es,zh')
  let useAllLanguages = $state(true)
  let detector = $state(null)
  let detectedLanguage = $state('')
  let confidenceLanguage = $state('en')
  let confidenceScore = $state(null)
  let allConfidences = $state([])
  let error = $state('')

  const sampleTexts = [
    { text: 'Hello, how are you?', lang: 'English' },
    { text: 'Bonjour, comment allez-vous?', lang: 'French' },
    { text: 'Hola, ¿cómo estás?', lang: 'Spanish' },
    { text: 'Guten Tag, wie geht es Ihnen?', lang: 'German' },
    { text: '你好，你好吗？', lang: 'Chinese' },
    { text: 'こんにちは、お元気ですか？', lang: 'Japanese' },
    { text: 'Привет, как дела?', lang: 'Russian' },
    { text: 'مرحبا، كيف حالك؟', lang: 'Arabic' },
  ]

  async function createDetector() {
    error = ''
    try {
      if (useAllLanguages) {
        detector = await createDetectorForAllLanguages()
      } else {
        detector = await createDetectorForLanguages(selectedLanguages)
      }
      detectedLanguage = ''
      confidenceScore = null
      allConfidences = []
    } catch (e) {
      error = e.toString()
    }
  }

  async function detectLang() {
    if (!detector) {
      error = 'Please create a detector first'
      return
    }
    error = ''
    try {
      const result = await detectLanguage(detector, textInput)
      detectedLanguage = result || 'Could not detect language'
    } catch (e) {
      error = e.toString()
    }
  }

  async function getConfidence() {
    if (!detector) {
      error = 'Please create a detector first'
      return
    }
    error = ''
    try {
      const result = await computeLanguageConfidence(detector, textInput, confidenceLanguage)
      confidenceScore = result
    } catch (e) {
      error = e.toString()
    }
  }

  async function getAllConfidences() {
    if (!detector) {
      error = 'Please create a detector first'
      return
    }
    error = ''
    try {
      const results = await computeLanguageConfidenceValues(detector, textInput)
      allConfidences = results.slice(0, 10) // Show top 10
    } catch (e) {
      error = e.toString()
    }
  }

  function useSample(text) {
    textInput = text
  }
</script>

<main class="container">
  <h1>🌍 Lingua Language Detector</h1>

  {#if error}
    <div class="error">
      <strong>⚠️ Error:</strong> {error}
      <button class="error-close" onclick={() => error = ''}>✕</button>
    </div>
  {/if}

  <div class="card">
    <h2>1. Create Detector</h2>
    <div class="row">
      <label>
        <input type="radio" bind:group={useAllLanguages} value={true} />
        All Languages (75)
      </label>
      <label>
        <input type="radio" bind:group={useAllLanguages} value={false} />
        Specific Languages
      </label>
    </div>
    {#if !useAllLanguages}
      <input
        type="text"
        bind:value={selectedLanguages}
        placeholder="en,fr,de,es"
        class="lang-input"
      />
      <p class="help">Enter comma-separated ISO 639-1 codes (e.g., en,fr,de,es,zh)</p>
    {/if}
    <LoadingButton onclick={createDetector} loading>
      {detector ? '✓ Recreate Detector' : 'Create Detector'}
    </LoadingButton>
    {#if detector}
      <div class="status-success">✓ Detector ready</div>
    {/if}
  </div>

  <div class="card">
    <h2>2. Enter Text</h2>
    <textarea
      bind:value={textInput}
      placeholder="Enter text to detect language..."
      rows="4"
    ></textarea>
    
    <h3>Sample Texts:</h3>
    <div class="samples">
      {#each sampleTexts as sample}
        <LoadingButton class="sample-btn" variant="secondary" onclick={() => useSample(sample.text)}>
          {sample.lang}
        </LoadingButton>
      {/each}
    </div>
  </div>

  <div class="card">
    <h2>3. Detect Language</h2>
    {#if !detector}
      <p class="help">⚠️ Please create a detector first (Step 1)</p>
    {/if}
    <LoadingButton onclick={detectLang} disabled={!detector} loading>
      Detect Language
    </LoadingButton>
    {#if detectedLanguage}
      <div class="result">
        <strong>Detected Language:</strong> <span class="lang-code">{detectedLanguage}</span>
      </div>
    {/if}
  </div>

  <div class="card">
    <h2>4. Get Confidence for Specific Language</h2>
    {#if !detector}
      <p class="help">⚠️ Please create a detector first (Step 1)</p>
    {/if}
    <div class="row">
      <input
        type="text"
        bind:value={confidenceLanguage}
        placeholder="en"
        class="small-input"
      />
      <LoadingButton onclick={getConfidence} disabled={!detector} loading>
        Get Confidence
      </LoadingButton>
    </div>
    {#if confidenceScore !== null}
      <div class="result">
        <strong>Confidence for {confidenceLanguage.toUpperCase()}:</strong> 
        <span class="confidence">{(confidenceScore * 100).toFixed(2)}%</span>
      </div>
    {/if}
  </div>

  <div class="card">
    <h2>5. Get All Confidence Values</h2>
    {#if !detector}
      <p class="help">⚠️ Please create a detector first (Step 1)</p>
    {/if}
    <LoadingButton onclick={getAllConfidences} disabled={!detector} loading>
      Get All Confidences
    </LoadingButton>
    {#if allConfidences.length > 0}
      <div class="confidence-table">
        <table>
          <thead>
            <tr>
              <th>Rank</th>
              <th>Language</th>
              <th>Confidence</th>
              <th>Bar</th>
            </tr>
          </thead>
          <tbody>
            {#each allConfidences as conf, i}
              <tr>
                <td>{i + 1}</td>
                <td class="lang-code">{conf.language}</td>
                <td>{(conf.confidence * 100).toFixed(2)}%</td>
                <td>
                  <div class="bar-container">
                    <div class="bar" style="width: {conf.confidence * 100}%"></div>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</main>

<style>
  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }

  h1 {
    text-align: center;
    margin-bottom: 2rem;
  }

  .card {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
  }

  h2 {
    margin-top: 0;
    font-size: 1.2rem;
    color: #61dafb;
  }

  h3 {
    font-size: 1rem;
    margin-bottom: 0.5rem;
  }

  .row {
    display: flex;
    gap: 1rem;
    align-items: center;
    margin-bottom: 1rem;
  }

  label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  input[type="text"],
  textarea {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #444;
    border-radius: 4px;
    background: #1a1a1a;
    color: white;
    font-family: inherit;
  }

  .lang-input {
    margin-bottom: 0.5rem;
  }

  .small-input {
    width: 100px;
  }

  .help {
    font-size: 0.9rem;
    color: #888;
    margin-top: 0.25rem;
    margin-bottom: 1rem;
  }

  textarea {
    resize: vertical;
    min-height: 100px;
  }

  .samples {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  :global(.sample-btn) {
    padding: 0.4rem 0.8rem !important;
    font-size: 0.9rem;
    min-width: auto !important;
  }

  .result {
    margin-top: 1rem;
    padding: 1rem;
    background: rgba(97, 218, 251, 0.1);
    border-radius: 4px;
    border-left: 3px solid #61dafb;
  }

  .lang-code {
    font-family: monospace;
    font-weight: bold;
    color: #61dafb;
    font-size: 1.1rem;
  }

  .confidence {
    font-size: 1.2rem;
    color: #4ade80;
    font-weight: bold;
  }

  .confidence-table {
    margin-top: 1rem;
    overflow-x: auto;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid #333;
  }

  th {
    background: rgba(97, 218, 251, 0.1);
    color: #61dafb;
    font-weight: 600;
  }

  .bar-container {
    width: 100%;
    height: 20px;
    background: #333;
    border-radius: 4px;
    overflow: hidden;
  }

  .bar {
    height: 100%;
    background: linear-gradient(90deg, #61dafb, #4ade80);
    transition: width 0.3s;
  }

  .error {
    background: rgba(239, 68, 68, 0.2);
    border: 1px solid #ef4444;
    border-left: 3px solid #ef4444;
    padding: 1rem 3rem 1rem 1rem;
    border-radius: 4px;
    color: #fca5a5;
    margin-bottom: 1.5rem;
    position: relative;
    animation: slideDown 0.3s ease-out;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .error-close {
    position: absolute;
    top: 0.75rem;
    right: 0.75rem;
    background: none;
    border: none;
    color: #fca5a5;
    font-size: 1.2rem;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    line-height: 1;
    transition: all 0.2s;
  }

  .error-close:hover {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.2);
  }

  .status-success {
    margin-top: 1rem;
    padding: 0.75rem;
    background: rgba(74, 222, 128, 0.1);
    border-left: 3px solid #4ade80;
    border-radius: 4px;
    color: #4ade80;
    font-weight: 500;
  }
</style>
