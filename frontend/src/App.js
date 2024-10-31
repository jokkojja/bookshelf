import React from "react";
import './App.css'; // Импортируем CSS файл
import BookList from "./BookList";
import AuthorList from "./AuthorList";
import GenreList from "./GenreList";
import BookDetail from "./BookDetail";

function App() {
  return (
    <div className="container">
      <BookList />
      <h2>Authors</h2>
      <AuthorList />
      <h2>Genres</h2>
      <GenreList />
      <h2>Book Detail</h2>
      <BookDetail />
    </div>
  );
}

export default App;
