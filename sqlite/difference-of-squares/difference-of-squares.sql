-- Schema: CREATE TABLE "difference-of-squares" ("number" INT", property" TEXT, "result" INT);
-- Task: update the difference-of-squares table and set the result based on the number and property fields.
-- select distinct property from "difference-of-squares";
update "difference-of-squares"
set result = case property
when "squareOfSum" then pow(number * (number + 1) / 2, 2)
when "sumOfSquares" then number * (number + 1) * (2 * number + 1) / 6
when "differenceOfSquares" then pow(number * (number + 1) / 2, 2) - number * (number + 1) * (2 * number + 1) / 6
else result -- don't nuke existing result!
end;

