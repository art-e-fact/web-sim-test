// import init, { run_bevy_app, run_models_demo_app } from "rust-web-sim";
import init, { greet, foo } from "simulator";
import run_demo_scene from "rust-web-sim"
import "./App.css";

function App() {
  const runBevyApp = async () => {
    console.log("Running Bevy App");
    try {
      // run_bevy_app();
      // run_models_demo_app();
      // console.log("Bevy App ran");
      
      await init();
      greet();
      foo();
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
      <canvas id="kesko-wasm" width="1280" height="720">
        Your browser does not support the HTML5 canvas tag.
      </canvas>
    </div>
  );
}

export default App;
