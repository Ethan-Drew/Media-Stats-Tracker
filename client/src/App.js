import React, { useEffect, useState } from "react";
import './App.css';

function App() {
  const [data, setData] = useState(null);

  useEffect(() => {
    fetch("/json")
      .then((response) => {
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json();
      })
      .then((json) => setData(json.message))
      .catch((error) => console.error("Error fetching data:", error));
  }, []);

  return (
    <div className="App">
      <h1>React Client</h1>
      <p>API Response: {data ? data : "Loading..."}</p>
    </div>
  );
}

export default App;
