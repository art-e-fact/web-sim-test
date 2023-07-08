import init, { run_bevy_app } from "rust-web-sim";
import "./App.css";

function App() {
  const runBevyApp = async () => {
    console.log("Running Bevy App");
    await init();
    try {
      run_bevy_app();
    } catch (error: any) {
      if (
        !error.message.includes(
          "Using exceptions for control flow, don't mind me. This isn't actually an error!"
        )
      ) {
        throw error;
      }
    }
  };

  return (
    <div className="App">
      <button onClick={runBevyApp}>Run Bevy App</button>
    </div>
  );
}

export default App;
