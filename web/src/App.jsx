import { useState, useEffect } from 'react'
import './App.css'

function App() {
  const [teams, setTeams] = useState('')
  const [courtNum, setCourtNum] = useState('')
  const [result, setResult] = useState(null)
  const [wasmModule, setWasmModule] = useState(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)

  useEffect(() => {
    // Load WebAssembly module
    const loadWasm = async () => {
      try {
        const wasm = await import('../../pkg/round_robin_tournament.js')
        await wasm.default()
        setWasmModule(wasm)
      } catch (err) {
        console.error('Failed to load WASM:', err)
        setError('WebAssemblyモジュールの読み込みに失敗しました')
      }
    }
    loadWasm()
  }, [])

  const generateTournament = () => {
    if (!wasmModule) {
      setError('WebAssemblyモジュールがまだ読み込まれていません')
      return
    }

    if (!teams.trim()) {
      setError('チーム名を入力してください')
      return
    }

    const courtNumValue = parseInt(courtNum, 10)
    if (!courtNum || isNaN(courtNumValue) || courtNumValue < 1) {
      setError('コート数は1以上を入力してください')
      return
    }

    setLoading(true)
    setError(null)

    try {
      const resultJson = wasmModule.generate_tournament(teams, courtNumValue)
      const parsedResult = JSON.parse(resultJson)
      setResult(parsedResult)
    } catch (err) {
      console.error('Tournament generation error:', err)
      setError('トーナメントの生成に失敗しました: ' + err.message)
    } finally {
      setLoading(false)
    }
  }

  const downloadCSV = () => {
    if (!result) return

    let csv = '\uFEFF' // UTF-8 BOM
    
    // Header
    const maxCourts = Math.max(...result.rounds.map(r => r.matches.length))
    csv += ',' + Array.from({ length: maxCourts }, (_, i) => `第${i + 1}コート`).join(',') + '\n'
    
    // Data rows
    result.rounds.forEach(round => {
      const matchStrings = round.matches.map(match => `${match.team1}-${match.team2}`)
      csv += `第${round.round_number}試合,` + matchStrings.join(',') + '\n'
    })

    const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' })
    const link = document.createElement('a')
    link.href = URL.createObjectURL(blob)
    link.download = '組合せ結果.csv'
    link.click()
  }

  return (
    <div className="app">
      <h1>総当り戦試合表作成</h1>
      
      <div className="input-section">
        <div className="form-group">
          <label htmlFor="teams">
            チーム名（改行区切りで入力）:
          </label>
          <textarea
            id="teams"
            value={teams}
            onChange={(e) => setTeams(e.target.value)}
            placeholder="チームA&#10;チームB&#10;チームC&#10;チームD"
            rows={10}
            disabled={loading}
          />
        </div>

        <div className="form-group">
          <label htmlFor="courtNum">
            コート数:
          </label>
          <input
            type="text"
            id="courtNum"
            value={courtNum}
            onChange={(e) => {
              const value = e.target.value;
              // Allow empty string or numeric input only
              if (value === '' || /^\d+$/.test(value)) {
                setCourtNum(value);
              }
            }}
            disabled={loading}
          />
        </div>

        <button 
          onClick={generateTournament}
          disabled={loading || !wasmModule}
          className="generate-btn"
        >
          {loading ? '生成中...' : '試合表作成'}
        </button>
      </div>

      {error && (
        <div className="error">
          {error}
        </div>
      )}

      {result && result.rounds && result.rounds.length > 0 && (
        <div className="result-section">
          <div className="result-header">
            <h2>組合せ結果</h2>
            <button onClick={downloadCSV} className="download-btn">
              CSV ダウンロード
            </button>
          </div>
          
          <div className="table-container">
            <table>
              <thead>
                <tr>
                  <th>試合</th>
                  {result.rounds[0].matches.map((_, idx) => (
                    <th key={idx}>第{idx + 1}コート</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {result.rounds.map((round) => (
                  <tr key={round.round_number}>
                    <td>第{round.round_number}試合</td>
                    {round.matches.map((match, idx) => (
                      <td key={idx}>
                        {match.team1} - {match.team2}
                      </td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  )
}

export default App
