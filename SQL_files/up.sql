CREATE TABLE "wards"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"ward_name" VARCHAR NOT NULL
);

CREATE TABLE "streets"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"ward_id" INTEGER NOT NULL,
	"street_name" VARCHAR NOT NULL,
	FOREIGN KEY ("ward_id") REFERENCES "wards"("id")
);

CREATE TABLE "houses"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"street_id" INTEGER NOT NULL,
	"house_number" VARCHAR NOT NULL,
	FOREIGN KEY ("street_id") REFERENCES "streets"("id")
);

CREATE TABLE "house_visits"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"house_id" INTEGER NOT NULL,
	"timestamp" TIMESTAMP NOT NULL,
	"voter_count" INTEGER,
	"yp_voters" INTEGER,
	"lab_voters" INTEGER,
	"lib_voters" INTEGER,
	"con_voters" INTEGER,
	"ref_voters" INTEGER,
	"grn_voters" INTEGER,
	FOREIGN KEY ("house_id") REFERENCES "houses"("id")
);