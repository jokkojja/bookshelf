// GenreList.js
import React, { useEffect, useState } from "react";

function GenreList() {
    const [genres, setGenres] = useState([]);
    const [error, setError] = useState(null);

    const fetchGenres = () => {
        fetch("http://localhost:3000/api/v1/genres")
            .then((response) => {
                if (!response.ok) {
                    throw new Error("Network response was not ok");
                }
                return response.json();
            })
            .then((data) => {
                setGenres(data.genres);
                setError(null); // Сброс ошибки при успешном запросе
            })
            .catch((error) => {
                setError(error.message);
            });
    };

    useEffect(() => {
        fetchGenres();
    }, []);

    if (error) {
        return <div>Error: {error}</div>;
    }

    return (
        <div>
            <button onClick={fetchGenres}>Reload Genres</button>
            {genres.length === 0 ? (
                <p>No genres available.</p>
            ) : (
                <ul>
                    {genres.map((genre, index) => (
                        <li key={index}>{genre.genre}</li>
                    ))}
                </ul>
            )}
        </div>
    );
}

export default GenreList;
