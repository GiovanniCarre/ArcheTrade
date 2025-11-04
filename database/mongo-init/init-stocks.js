db = db.getSiblingDB('marketDB');

db.createCollection('stocks');

db.stocks.createIndex(
    { symbol: 1 },
    { unique: true, name: "unique_symbol_index" }
);