CREATE TABLE IF NOT EXISTS "Instruments" (
    "InstrumentId" SERIAL PRIMARY KEY NOT NULL,
    "ExchangeCode" text NOT NULL,
    "ExchangePairCode" text NOT NULL,
    "BaseAssetId" integer,
    "QuoteAssetId" integer,
    "LegacySymbol" text,
    "Class" text,
    "TradeCount" bigint NOT NULL,
    "TradeCompressedSize" bigint NOT NULL
);

CREATE TABLE IF NOT EXISTS "Assets" (
    "Id" SERIAL PRIMARY KEY NOT NULL,
    "Code" text NOT NULL
);
