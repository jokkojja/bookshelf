import React, { useEffect, useState } from "react";

function BookDetail() {
    const [bookId, setBookId] = useState("");
    const [book, setBook] = useState(null);
    const [error, setError] = useState(null);

    const fetchBook = () => {
        if (!bookId) {
            setError("Please enter a book ID.");
            return;
        }

        fetch(`http://localhost:3000/api/v1/books/${bookId}`)
            .then((response) => {
                if (!response.ok) {
                    throw new Error("Network response was not ok");
                }
                return response.json();
            })
            .then((data) => {
                setBook(data);
                setError(null); // Сброс ошибки при успешном запросе
            })
            .catch((error) => {
                setError(error.message);
            });
    };

    return (
        <div>
            <h2>Get Book by ID</h2>
            <input
                type="text"
                value={bookId}
                onChange={(e) => setBookId(e.target.value)}
                placeholder="Enter book ID"
            />
            <button onClick={fetchBook}>Fetch Book</button>

            {error && <div style={{ color: "red" }}>Error: {error}</div>}

            {book ? (
                <>
                    <h3>{book.title}</h3>
                    <p>
                        Author: {book.author.first_name} {book.author.last_name}
                    </p>
                    <p>Genre: {book.genre}</p>
                </>
            ) : (
                <p>Loading book details...</p>
            )}
        </div>
    );
}

export default BookDetail;
