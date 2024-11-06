// AuthorList.js
import React, { useEffect, useState } from "react";

function AuthorList() {
    const [authors, setAuthors] = useState([]);
    const [error, setError] = useState(null);

    const fetchAuthors = () => {
        fetch("http://localhost:3000/api/v1/authors")
            .then((response) => {
                if (!response.ok) {
                    throw new Error("Network response was not ok");
                }
                return response.json();
            })
            .then((data) => {
                setAuthors(data.authors);
                setError(null); // Сброс ошибки при успешном запросе
            })
            .catch((error) => {
                setError(error.message);
            });
    };

    useEffect(() => {
        fetchAuthors();
    }, []);

    if (error) {
        return <div>Error: {error}</div>;
    }

    return (
        <div>
            <button onClick={fetchAuthors}>Reload Authors</button>
            {authors.length === 0 ? (
                <p>No authors available.</p>
            ) : (
                <ul>
                    {authors.map((author, index) => (
                        <li key={index}>
                            {author.first_name} {author.last_name}
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
}

export default AuthorList;
