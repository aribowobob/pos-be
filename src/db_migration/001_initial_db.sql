-- -------------------------------------------------------------
-- TablePlus 6.3.2(586)
--
-- https://tableplus.com/
--
-- Database: pos_db
-- Generation Time: 2025-05-28 14:42:00.4070
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS companies_id_seq;

-- Table Definition
CREATE TABLE "public"."companies" (
    "id" int4 NOT NULL DEFAULT nextval('companies_id_seq'::regclass),
    "name" varchar,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "updated_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS product_categories_id_seq;

-- Table Definition
CREATE TABLE "public"."product_categories" (
    "id" int4 NOT NULL DEFAULT nextval('product_categories_id_seq'::regclass),
    "name" varchar(255) NOT NULL,
    "description" text,
    "parent_id" int4,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS products_id_seq;

-- Table Definition
CREATE TABLE "public"."products" (
    "id" int4 NOT NULL DEFAULT nextval('products_id_seq'::regclass),
    "sku" varchar NOT NULL DEFAULT ''::character varying,
    "name" varchar NOT NULL DEFAULT ''::character varying,
    "purchase_price" numeric NOT NULL DEFAULT 0,
    "sale_price" numeric NOT NULL DEFAULT 0,
    "company_id" int4 NOT NULL CHECK (company_id > 0),
    "unit_name" varchar,
    "deleted_at" timestamp,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "updated_at" timestamp NOT NULL DEFAULT now(),
    "category_id" int4,
    PRIMARY KEY ("id")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS sales_cart_id_seq;

-- Table Definition
CREATE TABLE "public"."sales_cart" (
    "id" int4 NOT NULL DEFAULT nextval('sales_cart_id_seq'::regclass),
    "user_id" int4 NOT NULL,
    "store_id" int4 NOT NULL,
    "product_id" int4 NOT NULL,
    "base_price" numeric NOT NULL DEFAULT 0,
    "qty" int4 NOT NULL DEFAULT 0,
    "discount_type" varchar(10) NOT NULL DEFAULT 'fixed'::character varying CHECK ((discount_type)::text = ANY ((ARRAY['fixed'::character varying, 'percentage'::character varying])::text[])),
    "discount_value" int4 NOT NULL DEFAULT 0,
    "discount_amount" numeric NOT NULL DEFAULT 0,
    "sale_price" numeric NOT NULL DEFAULT 0,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "updated_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS sales_order_details_id_seq;

-- Table Definition
CREATE TABLE "public"."sales_order_details" (
    "id" int4 NOT NULL DEFAULT nextval('sales_order_details_id_seq'::regclass),
    "order_id" int4 NOT NULL,
    "product_id" int4 NOT NULL,
    "qty" int4 NOT NULL DEFAULT 0,
    "base_price" numeric NOT NULL DEFAULT 0,
    "discount_type" varchar(10) NOT NULL DEFAULT 'fixed'::character varying CHECK ((discount_type)::text = ANY ((ARRAY['fixed'::character varying, 'percentage'::character varying])::text[])),
    "discount_value" numeric NOT NULL DEFAULT 0,
    "discount_amount" numeric NOT NULL DEFAULT 0,
    "sale_price" numeric NOT NULL DEFAULT 0,
    "total_price" numeric NOT NULL DEFAULT 0,
    PRIMARY KEY ("id")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS sales_orders_id_seq;

-- Table Definition
CREATE TABLE "public"."sales_orders" (
    "id" int4 NOT NULL DEFAULT nextval('sales_orders_id_seq'::regclass),
    "order_number" bpchar(16) NOT NULL DEFAULT ''::bpchar,
    "user_id" int4 NOT NULL,
    "store_id" int4 NOT NULL,
    "date" date NOT NULL DEFAULT now(),
    "grand_total" numeric NOT NULL DEFAULT 0,
    "payment_cash" numeric NOT NULL DEFAULT 0,
    "payment_non_cash" numeric NOT NULL DEFAULT 0,
    "receivable" numeric NOT NULL DEFAULT 0,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "customer_id" int4,
    PRIMARY KEY ("id")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."stock" (
    "store_id" int4 NOT NULL CHECK (store_id > 0),
    "product_id" int4 NOT NULL CHECK (product_id > 0),
    "qty" int4 NOT NULL DEFAULT 0 CHECK (qty >= 0),
    "created_at" timestamp NOT NULL DEFAULT now(),
    "updated_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("store_id","product_id")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS stores_id_seq;

-- Table Definition
CREATE TABLE "public"."stores" (
    "id" int4 NOT NULL DEFAULT nextval('stores_id_seq'::regclass),
    "name" varchar NOT NULL DEFAULT ''::character varying,
    "company_id" int4 NOT NULL,
    "initial" varchar(3) NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "updated_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."user_features" (
    "user_id" int4 NOT NULL DEFAULT 0,
    "feature_code" bpchar(5) NOT NULL DEFAULT ''::bpchar,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "updated_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("user_id","feature_code")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."user_stores" (
    "user_id" int4 NOT NULL CHECK (user_id > 0),
    "store_id" int4 NOT NULL CHECK (store_id > 0),
    "created_at" timestamp NOT NULL DEFAULT now(),
    "updated_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("user_id","store_id")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS users_id_seq;

-- Table Definition
CREATE TABLE "public"."users" (
    "id" int4 NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    "email" varchar NOT NULL DEFAULT ''::character varying,
    "company_id" int4 NOT NULL CHECK (company_id > 0),
    "full_name" varchar NOT NULL DEFAULT ''::character varying,
    "initial" varchar(3) NOT NULL DEFAULT ''::bpchar,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "updated_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

INSERT INTO "public"."companies" ("id", "name", "created_at", "updated_at") VALUES
(1, 'Primadona', '2025-03-02 22:16:25.720533', '2025-03-02 22:16:25.720533');

INSERT INTO "public"."product_categories" ("id", "name", "description", "parent_id", "created_at", "updated_at") VALUES
(1, 'Electronics', 'Electronic devices and accessories', NULL, '2025-05-28 05:37:34.879057', '2025-05-28 05:37:34.879057'),
(2, 'Clothing', 'Apparel and fashion items', NULL, '2025-05-28 05:37:34.879057', '2025-05-28 05:37:34.879057'),
(3, 'Food & Beverage', 'Consumable products', NULL, '2025-05-28 05:37:34.879057', '2025-05-28 05:37:34.879057'),
(4, 'Home & Office', 'Items for home and office', NULL, '2025-05-28 05:37:34.879057', '2025-05-28 05:37:34.879057'),
(5, 'Smartphones', 'Mobile phones and accessories', 1, '2025-05-28 05:37:34.879057', '2025-05-28 05:37:34.879057'),
(6, 'Laptops', 'Notebook computers', 1, '2025-05-28 05:37:34.879057', '2025-05-28 05:37:34.879057'),
(7, 'T-shirts', 'Casual t-shirts', 2, '2025-05-28 05:37:34.879057', '2025-05-28 05:37:34.879057'),
(8, 'Beverages', 'Drinks and beverages', 3, '2025-05-28 05:37:34.879057', '2025-05-28 05:37:34.879057');

INSERT INTO "public"."stores" ("id", "name", "company_id", "initial", "created_at", "updated_at") VALUES
(1, 'Gudang HQ', 1, 'GUD', '2025-03-02 22:20:27.185994', '2025-03-02 22:20:27.185994'),
(2, 'Store Denpasar', 1, 'DPS', '2025-03-02 22:20:27.185994', '2025-03-02 22:20:27.185994'),
(3, 'Store Tabanan', 1, 'TAB', '2025-03-02 22:20:27.185994', '2025-03-02 22:20:27.185994');

INSERT INTO "public"."user_stores" ("user_id", "store_id", "created_at", "updated_at") VALUES
(1, 1, '2025-03-02 22:22:03.060724', '2025-03-02 22:22:03.060724'),
(1, 2, '2025-03-02 22:22:03.060724', '2025-03-02 22:22:03.060724'),
(1, 3, '2025-03-02 22:22:03.060724', '2025-03-02 22:22:03.060724');

INSERT INTO "public"."users" ("id", "email", "company_id", "full_name", "initial", "created_at", "updated_at") VALUES
(1, 'aribowo.susetyo@gmail.com', 1, 'ARIBOWO', 'ARI', '2025-03-02 22:17:08.298316', '2025-03-02 22:17:08.298316');
