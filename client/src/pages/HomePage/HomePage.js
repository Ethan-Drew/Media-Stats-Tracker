import React, { useEffect, useState } from 'react';
import './HomePage.css';

const HomePage = () => {
    const [data, setData] = useState(null);

    useEffect(() => {
        fetch("http://localhost:8000/json") // Update the URL to match your server
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
        <div className="home-page">
            <h1>Welcome to TrackMyMedia</h1>
            <p>API Response: {data ? data : "Loading..."}</p>
        </div>
    );
};

export default HomePage;
