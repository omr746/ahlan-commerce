--
-- PostgreSQL database dump
--

\restrict zacdsemCoeJFaKRqr5VaelycbPegZ0u2DJutjHI9mtu0nOKsmpMASnv8CLwFo3x

-- Dumped from database version 16.15 (Debian 16.15-1.pgdg13+2)
-- Dumped by pg_dump version 16.15 (Debian 16.15-1.pgdg13+2)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: atlas_schema_revisions; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA atlas_schema_revisions;


ALTER SCHEMA atlas_schema_revisions OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: atlas_schema_revisions; Type: TABLE; Schema: atlas_schema_revisions; Owner: postgres
--

CREATE TABLE atlas_schema_revisions.atlas_schema_revisions (
    version character varying NOT NULL,
    description character varying NOT NULL,
    type bigint DEFAULT 2 NOT NULL,
    applied bigint DEFAULT 0 NOT NULL,
    total bigint DEFAULT 0 NOT NULL,
    executed_at timestamp with time zone NOT NULL,
    execution_time bigint NOT NULL,
    error text,
    error_stmt text,
    hash character varying NOT NULL,
    partial_hashes jsonb,
    operator_version character varying NOT NULL
);


ALTER TABLE atlas_schema_revisions.atlas_schema_revisions OWNER TO postgres;

--
-- Name: import_jobs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.import_jobs (
    id uuid NOT NULL,
    status text NOT NULL,
    input_path text NOT NULL,
    attempts integer DEFAULT 0 NOT NULL,
    last_error text,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    CONSTRAINT import_jobs_attempts_nonneg_check CHECK ((attempts >= 0)),
    CONSTRAINT import_jobs_status_check CHECK ((status = ANY (ARRAY['queued'::text, 'running'::text, 'succeeded'::text, 'failed'::text])))
);


ALTER TABLE public.import_jobs OWNER TO postgres;

--
-- Name: products; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.products (
    id uuid NOT NULL,
    title text NOT NULL,
    handle text NOT NULL,
    description text,
    price_cents integer NOT NULL,
    inventory_quantity integer NOT NULL,
    published boolean NOT NULL,
    published_at timestamp with time zone,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


ALTER TABLE public.products OWNER TO postgres;

--
-- Name: refinery_schema_history; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.refinery_schema_history (
    version integer NOT NULL,
    name character varying(255),
    applied_on character varying(255),
    checksum character varying(255)
);


ALTER TABLE public.refinery_schema_history OWNER TO postgres;

--
-- Data for Name: atlas_schema_revisions; Type: TABLE DATA; Schema: atlas_schema_revisions; Owner: postgres
--

COPY atlas_schema_revisions.atlas_schema_revisions (version, description, type, applied, total, executed_at, execution_time, error, error_stmt, hash, partial_hashes, operator_version) FROM stdin;
20260823133846	initial_products	2	1	1	2026-08-24 16:58:58.167512+00	1894547			PJBWN0LCmk0RLmbU6AgZmlZbzgWTZxYiQCllNw1U+pg=	null	Atlas CLI v1.3.2-7a1b9e3-canary
20260826164149	make_published_at_nullable	2	1	1	2026-08-26 16:42:11.551784+00	1535732			9+FPQraJbRUOp73oHSjeOD/7W2Q/dCZXJ/oLf3DhZnY=	null	Atlas CLI v1.3.2-7a1b9e3-canary
20260826164340	make_published_not_null	2	1	1	2026-08-26 16:43:45.359044+00	1407177			oVWaNrbuwZaN0Z/9/ywoQqEdmkT5pYlGZtEN+u251tg=	null	Atlas CLI v1.3.2-7a1b9e3-canary
20260913064151	add_import_jobs	2	2	2	2026-09-13 06:42:10.310972+00	1480089			hweM8AzYpqPp8WKCOvqSmmI8Rgarbr+N/B2jX8kHKZM=	null	Atlas CLI v1.3.2-7a1b9e3-canary
\.


--
-- Data for Name: import_jobs; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.import_jobs (id, status, input_path, attempts, last_error, created_at, updated_at) FROM stdin;
01a09acb-1321-7db2-ba57-c3437dd6b137	succeeded	fixtures/products.json	1	\N	2026-09-13 12:43:15.10542+00	2026-09-13 12:43:15.453782+00
01a09aca-1580-79e2-9d5a-ecc5acd3d345	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:42:10.176644+00	2026-09-13 13:42:47.910579+00
01a09acd-a4ea-7bf1-a426-ddbdefa2f817	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:46:03.498168+00	2026-09-13 13:42:47.925774+00
01a09ace-568b-7173-ac1b-572da8f2a42e	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:46:48.971537+00	2026-09-13 13:42:47.940095+00
01a09ace-fb0d-7192-a9f6-0da0c1bc40f6	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:47:31.085432+00	2026-09-13 13:42:47.979698+00
01a09ad0-3b6d-7962-849d-b63e9a39611e	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:48:53.101387+00	2026-09-13 13:42:47.994308+00
01a09ad2-0df3-7273-9f87-5991455ca29f	succeeded	fixtures/products.json	1	\N	2026-09-13 12:50:52.531176+00	2026-09-13 12:50:52.821125+00
01a09ad0-a84b-7791-b84f-7b818627869b	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:49:20.971738+00	2026-09-13 13:42:48.007969+00
01a09ad2-4476-7481-9d73-94bcce70f813	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:51:06.48659+00	2026-09-13 13:42:48.023153+00
01a09ad5-6973-7c80-90c0-5d0ba94424ba	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:54:32.563837+00	2026-09-13 13:42:48.037764+00
01a09ad5-d44c-7a33-bb9b-eca453494001	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:54:59.916017+00	2026-09-13 13:42:48.054006+00
01a09ad6-1c45-7d02-9d0a-7273008fd563	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:55:18.341208+00	2026-09-13 13:42:48.201463+00
01a09ad6-361d-7f13-9104-0e20ea47538f	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 12:55:24.957052+00	2026-09-13 13:42:48.235155+00
01a09aef-4c08-7f02-adab-a6ef575b06a7	failed	fixtures/products.json	3	product handle 'product handle' is already in use 	2026-09-13 13:22:48.968081+00	2026-09-13 13:42:48.249136+00
01a09aef-230d-7df1-8608-c65b7fdd0db8	succeeded	fixtures/products.json	1	\N	2026-09-13 13:22:38.477371+00	2026-09-13 13:22:39.144442+00
01a09b00-3437-7b83-ae87-988160b62464	succeeded	fixtures/products.json	1	\N	2026-09-13 13:41:16.983753+00	2026-09-13 13:41:17.772188+00
\.


--
-- Data for Name: products; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.products (id, title, handle, description, price_cents, inventory_quantity, published, published_at, created_at, updated_at) FROM stdin;
01a034b6-66d0-74a3-8a8a-7cc3a2ce7e15	Coffee Mug7	coffee-mug7	Ceramic mug for daily coffee.	2500	12	t	2026-08-27 15:11:50.664979+00	2026-08-24 16:59:24.240806+00	2026-08-27 15:11:50.664979+00
01a04e6f-6335-7b52-8743-aa666ca116da	Test Product	test-product-3539fd9e-dc3f-4123-840d-87d2771e5252	\N	1000	10	t	2026-08-29 16:51:57.877165+00	2026-08-29 16:51:57.877165+00	2026-08-29 16:51:57.877165+00
01a04e6f-6357-7812-a446-a008930429c9	T-Shirt	test-product-10780707-e211-457d-a810-dd1e5c8eb31d	\N	1999	5	t	2026-08-29 16:51:57.911018+00	2026-08-29 16:51:57.911018+00	2026-08-29 16:51:57.911018+00
01a04e72-ce11-7013-9817-707b1e5d8fe4	Test Product	test-product-7867185d-c491-485f-98a0-c8c7b9d3ef7e	\N	1000	10	t	2026-08-29 16:55:41.84141+00	2026-08-29 16:55:41.84141+00	2026-08-29 16:55:41.84141+00
01a04e72-ce11-7013-9817-70545a5a1589	T-Shirt	test-product-d8aab4cc-124e-4e65-85af-95d4c16f1963	\N	1999	5	t	2026-08-29 16:55:41.841119+00	2026-08-29 16:55:41.841119+00	2026-08-29 16:55:41.841119+00
01a04e77-33cd-7042-bb8d-1d380cc2ffa9	Coffee Mug	coffee-mug	\N	2500	12	t	2026-08-29 17:00:30.029734+00	2026-08-29 17:00:30.029734+00	2026-08-29 17:00:30.029734+00
01a04e83-f0fe-7970-b3ff-212ff3379a0f	Test Product	test-product-4655d49a-7916-473e-b755-e080f83071fe	\N	1000	10	t	2026-08-29 17:14:24.894178+00	2026-08-29 17:14:24.894178+00	2026-08-29 17:14:24.894178+00
01a04e83-f14f-70b2-a325-761d70585980	T-Shirt	test-product-e2628cdd-6b05-408d-b691-21d21c832226	\N	1999	5	t	2026-08-29 17:14:24.975752+00	2026-08-29 17:14:24.975752+00	2026-08-29 17:14:24.975752+00
01a04e8f-b377-7903-a300-d09534dae754	Original	prd-duplicate-b621a52e-b0cb-48c2-b4ba-37fc99b13ae6	\N	1200	5	t	2026-08-29 17:27:15.575132+00	2026-08-29 17:27:15.575132+00	2026-08-29 17:27:15.575132+00
01a04e8f-b377-7903-a300-d0a293abb592	Sun Hat	prd-valid-create-25850d65-6c04-42ca-b6cb-53b242d88392	\N	1200	5	t	2026-08-29 17:27:15.575132+00	2026-08-29 17:27:15.575132+00	2026-08-29 17:27:15.575132+00
01a04e8f-b377-7903-a300-d0c85b26c15e	Published Item	prd-list-published-156cfdb5-d4a8-4941-a023-761f657c79ed	\N	500	1	t	2026-08-29 17:27:15.575763+00	2026-08-29 17:27:15.575763+00	2026-08-29 17:27:15.575763+00
01a04e8f-b382-74c0-b5c5-32d9be6a5718	Draft Item	prd-list-draft-3c0715a0-e184-485f-b6e2-ffb978633e39	\N	500	1	f	\N	2026-08-29 17:27:15.586579+00	2026-08-29 17:27:15.586579+00
01a04e93-fa38-7903-9b53-5ee52d981db3	Coffee Mug4	coffee-mug4	\N	2500	12	f	\N	2026-08-29 17:31:55.83273+00	2026-08-29 17:31:55.83273+00
01a04e97-de67-7621-b65f-4e6925a0bbee	Published Item	prd-list-published-fa06d065-072b-4855-b4ad-5f6caf10f26f	\N	500	1	t	2026-08-29 17:36:10.85542+00	2026-08-29 17:36:10.85542+00	2026-08-29 17:36:10.85542+00
01a04e97-de67-7621-b65f-4e753db1412d	Sun Hat	prd-valid-create-154e9d35-3171-4bc4-a4dc-5ea1df5b78b6	\N	1200	5	t	2026-08-29 17:36:10.855564+00	2026-08-29 17:36:10.855564+00	2026-08-29 17:36:10.855564+00
01a04e97-de67-7621-b65f-4e5e727c2f65	Original	prd-duplicate-d29906b0-373d-4a7d-8f88-4bde80289c9c	\N	1200	5	t	2026-08-29 17:36:10.85542+00	2026-08-29 17:36:10.85542+00	2026-08-29 17:36:10.85542+00
01a04e97-de6d-73e0-b897-d83676a5db60	Draft Item	prd-list-draft-2aea0ac8-66fd-4228-8913-bc16845f9b4f	\N	500	1	f	\N	2026-08-29 17:36:10.862004+00	2026-08-29 17:36:10.862004+00
01a063bd-f716-7783-8e3d-8e1dfe1a6c7a	Laptop	laptop	Gaming laptop	50000	10	t	2026-09-02 20:09:49.078264+00	2026-09-02 20:09:49.078264+00	2026-09-02 20:09:49.078264+00
01a094b7-9bd7-7410-9176-2cb25a86c81a	jj	jj	djdj	89	90	f	\N	2026-09-12 08:24:16.087855+00	2026-09-12 08:24:16.087855+00
01a0954c-c416-79f2-acf9-3bc55b934c28	jj	jj2	djd	50	50	t	2026-09-12 11:07:11.254282+00	2026-09-12 11:07:11.254282+00	2026-09-12 11:07:11.254282+00
01a09554-8738-7782-a1f9-c40fbb866c14	E2E Test Product	e2e-test-product-1789211740901	\N	1999	10	f	\N	2026-09-12 11:15:39.960353+00	2026-09-12 11:15:39.960353+00
01a09acb-1477-7b20-a13d-7a285122d49a	Coffee Mug	coffee-mug10	Ceramic mug for daily coffee.	2500	12	t	2026-09-13 12:43:15.44747+00	2026-09-13 12:43:15.44747+00	2026-09-13 12:43:15.44747+00
01a09acb-147a-7d01-8877-27e310f6dfb8	Notebook	notebook2	Plain notebook for daily planning.	1200	30	f	\N	2026-09-13 12:43:15.450266+00	2026-09-13 12:43:15.450266+00
01a09ad2-0f0f-7da3-8c2a-2406be8944a4	Coffee Mug	coffee-mug111	Ceramic mug for daily coffee.	2500	12	t	2026-09-13 12:50:52.81542+00	2026-09-13 12:50:52.81542+00	2026-09-13 12:50:52.81542+00
01a09ad2-0f12-7c13-aef0-de5a71bede3e	Notebook	notebook341	Plain notebook for daily planning.	1200	30	f	\N	2026-09-13 12:50:52.81831+00	2026-09-13 12:50:52.81831+00
01a09aef-259b-7d40-b3c4-35e8bb5c875e	Coffee Mug	coffee-mug1111	Ceramic mug for daily coffee.	2500	12	t	2026-09-13 13:22:39.131274+00	2026-09-13 13:22:39.131274+00	2026-09-13 13:22:39.131274+00
01a09aef-25a4-7b90-a4dc-a5bb6bddf284	Notebook	notebook34171	Plain notebook for daily planning.	1200	30	f	\N	2026-09-13 13:22:39.140812+00	2026-09-13 13:22:39.140812+00
01a09b00-373f-7e40-a5e1-caa48ed65af6	Coffee Mug	coffee-mug11111	Ceramic mug for daily coffee.	2500	12	t	2026-09-13 13:41:17.759218+00	2026-09-13 13:41:17.759218+00	2026-09-13 13:41:17.759218+00
01a09b00-3747-79d1-81fe-dff41f2ab1e2	Notebook	notebook341715	Plain notebook for daily planning.	1200	30	f	\N	2026-09-13 13:41:17.767335+00	2026-09-13 13:41:17.767335+00
\.


--
-- Data for Name: refinery_schema_history; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.refinery_schema_history (version, name, applied_on, checksum) FROM stdin;
\.


--
-- Name: atlas_schema_revisions atlas_schema_revisions_pkey; Type: CONSTRAINT; Schema: atlas_schema_revisions; Owner: postgres
--

ALTER TABLE ONLY atlas_schema_revisions.atlas_schema_revisions
    ADD CONSTRAINT atlas_schema_revisions_pkey PRIMARY KEY (version);


--
-- Name: import_jobs import_jobs_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.import_jobs
    ADD CONSTRAINT import_jobs_pkey PRIMARY KEY (id);


--
-- Name: products products_handle_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_handle_key UNIQUE (handle);


--
-- Name: products products_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_pkey PRIMARY KEY (id);


--
-- Name: refinery_schema_history refinery_schema_history_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.refinery_schema_history
    ADD CONSTRAINT refinery_schema_history_pkey PRIMARY KEY (version);


--
-- Name: import_jobs_status_created_at_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX import_jobs_status_created_at_idx ON public.import_jobs USING btree (status, created_at);


--
-- PostgreSQL database dump complete
--

\unrestrict zacdsemCoeJFaKRqr5VaelycbPegZ0u2DJutjHI9mtu0nOKsmpMASnv8CLwFo3x

