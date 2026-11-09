import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { motion } from "framer-motion";
import { Check, Loader2, RotateCcw, Sparkles, Volume2 } from "lucide-react";

const fallbackLesson = {
  kannada: "ನಮಸ್ಕಾರ",
  transliteration: "Namaskara",
  english: "Hello / respectful greeting",
  explanation: "Use it when greeting someone politely, especially at the start of a conversation.",
  practice: "Say namaskara when you greet someone today.",
};

function App() {
  const [topic, setTopic] = useState("daily conversation");
  const [lesson, setLesson] = useState(fallbackLesson);
  const [status, setStatus] = useState("Ready to ask Gemma for a fresh lesson.");
  const [isLoading, setIsLoading] = useState(false);

  async function loadLesson() {
    setIsLoading(true);
    setStatus("Asking OpenRouter...");

    try {
      const nextLesson = await invoke("teach_kannada", { topic });
      setLesson(nextLesson);
      setStatus("Lesson loaded from google/gemma-4-31b-it:free.");
    } catch (error) {
      setStatus(String(error));
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <main className="shell">
      <motion.section
        className="lesson"
        initial={{ opacity: 0, y: 16, scale: 0.96 }}
        animate={{ opacity: 1, y: 0, scale: 1 }}
        transition={{ duration: 0.45, ease: [0.22, 1, 0.36, 1] }}
      >
        <header className="lesson-header">
          <div>
            <span className="eyebrow">Today in Kannada</span>
            <h1>{lesson.kannada}</h1>
          </div>
          <button className="icon-button" aria-label="Play pronunciation" title="Play pronunciation">
            <Volume2 size={19} />
          </button>
        </header>

        <div className="pronunciation">{lesson.transliteration}</div>
        <p className="meaning">{lesson.english}</p>
        <p className="note">{lesson.explanation}</p>
        <p className="practice">{lesson.practice}</p>

        <label className="topic-label" htmlFor="topic">
          Topic
        </label>
        <input
          id="topic"
          value={topic}
          onChange={(event) => setTopic(event.target.value)}
          placeholder="daily conversation"
        />

        <p className="status">{status}</p>

        <footer className="actions">
          <button className="primary" onClick={loadLesson} disabled={isLoading}>
            {isLoading ? <Loader2 className="spin" size={17} /> : <Sparkles size={17} />}
            Teach me
          </button>
          <button className="secondary" title="Get another lesson" onClick={loadLesson} disabled={isLoading}>
            <RotateCcw size={17} />
          </button>
          <button className="secondary" title="Practice this">
            <Check size={17} />
          </button>
        </footer>
      </motion.section>
    </main>
  );
}

export default App;
