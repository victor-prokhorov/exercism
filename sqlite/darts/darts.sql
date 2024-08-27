-- Schema: CREATE TABLE "darts" ("x" REAL, "y" REAL, score INTEGER);
-- Task: update the darts table and set the score based on the x and y values.
update darts
set score = case
when sqrt(pow(x, 2) + pow(y, 2)) > 10 then 0
when sqrt(pow(x, 2) + pow(y, 2)) > 5 then 1
when sqrt(pow(x, 2) + pow(y, 2)) > 1 then 5
else 10
end;
