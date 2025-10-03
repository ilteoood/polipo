This documents contains the technical requirements of this project.

The project will be made using the Rust language, to consume to lowest amount of resources as possible. Your must act as a senior Rust software engineer.

The flow, described in the business requirements, will need to run following a cronjob. Its interval must be configurable through an environment variable.

Once the cron is triggered, here are the technical steps that needs to be implemented:
- Fetch the content of the page https://octopusenergy.it/le-nostre-tariffe
- Extract the JSON content of the `script` tag with ID `__NEXT_DATA__`
The extracted content should look like
```
"{\"props\":{\"pageProps\":{\"products\":[{\"__typename\":\"ElectricityProductType\",\"code\":\"000129ESFML08XXXXXXXXOCTOFIXv108\",\"displayName\":\"Octopus Fissa 12M\",\"termsAndConditionsUrl\":\"https://s3.eu-south-1.amazonaws.com/oeit-prod-user-documents/terms-and-conditions/Octo%20Fissa%2012M%20EE%20Domestico-108-1.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256\\u0026X-Amz-Credential=ASIAS3MIUWL27HODJBZ7%2F20250915%2Feu-south-1%2Fs3%2Faws4_request\\u0026X-Amz-Date=20250915T070517Z\\u0026X-Amz-Expires=3600\\u0026X-Amz-SignedHeaders=host\\u0026X-Amz-Security-Token=IQoJb3JpZ2luX2VjEPf%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCmV1LXNvdXRoLTEiRzBFAiEA%2Byp6ZLeUneu9101BeB66DK3TGGAyFJBnLH7B2aqSxHUCIHZc8%2BVntirdiVvPmQjvUxVUmUq0PsYEqvhfwPl1dX38KoUFCHAQAhoMMTk2MjQ0NDUyMDg1IgwkMqNYzLQtKgzo0csq4gQjgd8W4GHMretmUzzp5A1VX%2FqviCGtAqdo%2BP42%2FisY1pyYn8X%2B6kbBEv6wF3P39GmBmChyGytsr2mN5dyJa3lLaqiulPgHQVFy4u%2FbR1NhjsPeYITiXE9YF9hNXjjbuzYmgVExNAfHjLWItCaCYUOdNlWItIC88Jghiur6VxMAD0n7Uc8ZlUAaE7yFvk9iuFrcgQpHQ7JkCYgtzwV68J84%2FL%2BSjBnf22ZZqGKsocA0nI0W9xHgxd4a8xndwHu6lwsXcylXfaEcjLMoOP5%2BTmsIna0ES%2FCPcOWmm31oMqaHT9e5b0d2J%2FbIZTvqZV7l5kHxmIdvQ89xBEZ7OnMGMsTVmxfnSMS7MN7kBtTvpi73CIr4NXHVwa6qbCcvuKfyNlqgiKrsuTt1%2Bhg53%2FUwixxt6bciWpqI%2FItEHNyX%2FW9df7jG7CVI4h%2BAj7zcTmreWkmeMFQ46BJseC5AX%2FoLc2pMX9k%2BuZjNtGin3Qde0ngF4q54l%2Bow78Epw9lLjmk5jmmaXCk5iCw1OG3FUQVwkxzqSc5QTBgSJFeuyTuS8niUG92dknSHxpbnzQYGnbjo2XQFnnsnAGt91am7et9TQpyb4%2FpBdoiuu%2FFb7jFNmgnrQwoTfuZClr7PkJzqaVqj3UeAMIypqY%2Fwt8C8AGUKrPgMlhYZXKLyGuX1Q5aCTwRHcf1A4TCfYrsrI%2BRrzm4JbU%2FQfrOeaaFCAwzj5bGkWAZIWXhefu6FrYuV2KWWxjhpP1%2F%2BDuw9fCaH9iZJWoXeCLoEOmZr83FywCsZITZU78qdYfI4o0wL%2Fi2DtyrZwv1PaFq8MP7znsYGOpoBkcY9ln244b5OEVUG%2BJKUZ%2BEukm0MstvVYb4Udx49RQbZ%2FxYUVU0uSaASThwSoAlsJF276DdWkYfeg0i9sB%2FzkSDYXyyZjFIgTeGKUBbTbxNNZWOY%2Bltaf7GdP4mgKdCbp6glhJdp2cg2wgYAwp2nBFzgl0VOMHFSXzZw58cYlSs9hRJqTIyPdV74RSw8RViP6bEh8WASfbxxhg%3D%3D\\u0026X-Amz-Signature=69d244c4115a55d6825e13b46b56ad0b9c27793a1efc7193323ede7512fe59fe\",\"fullName\":\"Octopus Fissa 12M\",\"description\":\"Con Octopus Fissa 12M, blocchi il prezzo della materia energia e dei costi di commercializzazione per un anno. Al termine dei 12 mesi potrai scegliere nuovamente la tariffa più conveniente per te (Fissa o Flex) e continuare a risparmiare, senza stress.\",\"params\":{\"productType\":\"FIXED_SINGLE_RATE\",\"annualStandingCharge\":\"84\",\"consumptionCharge\":\"0,1089\"}},{\"__typename\":\"ElectricityProductType\",\"code\":\"000129ESVFL89XXXXXXXXOCTOFLEXv89\",\"displayName\":\"Octopus Flex\",\"termsAndConditionsUrl\":\"https://s3.eu-south-1.amazonaws.com/oeit-prod-user-documents/terms-and-conditions/Octo%20Flex%20EE%20Domestico-89-1.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256\\u0026X-Amz-Credential=ASIAS3MIUWL27HODJBZ7%2F20250915%2Feu-south-1%2Fs3%2Faws4_request\\u0026X-Amz-Date=20250915T070517Z\\u0026X-Amz-Expires=3600\\u0026X-Amz-SignedHeaders=host\\u0026X-Amz-Security-Token=IQoJb3JpZ2luX2VjEPf%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCmV1LXNvdXRoLTEiRzBFAiEA%2Byp6ZLeUneu9101BeB66DK3TGGAyFJBnLH7B2aqSxHUCIHZc8%2BVntirdiVvPmQjvUxVUmUq0PsYEqvhfwPl1dX38KoUFCHAQAhoMMTk2MjQ0NDUyMDg1IgwkMqNYzLQtKgzo0csq4gQjgd8W4GHMretmUzzp5A1VX%2FqviCGtAqdo%2BP42%2FisY1pyYn8X%2B6kbBEv6wF3P39GmBmChyGytsr2mN5dyJa3lLaqiulPgHQVFy4u%2FbR1NhjsPeYITiXE9YF9hNXjjbuzYmgVExNAfHjLWItCaCYUOdNlWItIC88Jghiur6VxMAD0n7Uc8ZlUAaE7yFvk9iuFrcgQpHQ7JkCYgtzwV68J84%2FL%2BSjBnf22ZZqGKsocA0nI0W9xHgxd4a8xndwHu6lwsXcylXfaEcjLMoOP5%2BTmsIna0ES%2FCPcOWmm31oMqaHT9e5b0d2J%2FbIZTvqZV7l5kHxmIdvQ89xBEZ7OnMGMsTVmxfnSMS7MN7kBtTvpi73CIr4NXHVwa6qbCcvuKfyNlqgiKrsuTt1%2Bhg53%2FUwixxt6bciWpqI%2FItEHNyX%2FW9df7jG7CVI4h%2BAj7zcTmreWkmeMFQ46BJseC5AX%2FoLc2pMX9k%2BuZjNtGin3Qde0ngF4q54l%2Bow78Epw9lLjmk5jmmaXCk5iCw1OG3FUQVwkxzqSc5QTBgSJFeuyTuS8niUG92dknSHxpbnzQYGnbjo2XQFnnsnAGt91am7et9TQpyb4%2FpBdoiuu%2FFb7jFNmgnrQwoTfuZClr7PkJzqaVqj3UeAMIypqY%2Fwt8C8AGUKrPgMlhYZXKLyGuX1Q5aCTwRHcf1A4TCfYrsrI%2BRrzm4JbU%2FQfrOeaaFCAwzj5bGkWAZIWXhefu6FrYuV2KWWxjhpP1%2F%2BDuw9fCaH9iZJWoXeCLoEOmZr83FywCsZITZU78qdYfI4o0wL%2Fi2DtyrZwv1PaFq8MP7znsYGOpoBkcY9ln244b5OEVUG%2BJKUZ%2BEukm0MstvVYb4Udx49RQbZ%2FxYUVU0uSaASThwSoAlsJF276DdWkYfeg0i9sB%2FzkSDYXyyZjFIgTeGKUBbTbxNNZWOY%2Bltaf7GdP4mgKdCbp6glhJdp2cg2wgYAwp2nBFzgl0VOMHFSXzZw58cYlSs9hRJqTIyPdV74RSw8RViP6bEh8WASfbxxhg%3D%3D\\u0026X-Amz-Signature=6da7c180cfa6437d3895c69e2dcfe517a363cfb49a2e693cb9426e5d01f6f997\",\"fullName\":\"Octopus Flex Multi\",\"description\":\"La tariffa Octopus Flex prevede un prezzo per la materia energia indicizzato, ovvero sempre in linea con il mercato all’ingrosso. I costi di commercializzazione sono invece bloccati per sempre, per una bolletta senza sorprese.\",\"params\":{\"productType\":\"VARIABLE\",\"annualStandingCharge\":\"84\",\"consumptionCharge\":\"0,0088\"}},{\"__typename\":\"ElectricityProductType\",\"code\":\"000129ESVML75XXXXXOCTOFLEXMONv75\",\"displayName\":\"Octopus Flex Mono\",\"termsAndConditionsUrl\":\"https://s3.eu-south-1.amazonaws.com/oeit-prod-user-documents/terms-and-conditions/Octopus%20Flex%20Mono%20EE%20Domestico-75-1.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256\\u0026X-Amz-Credential=ASIAS3MIUWL27HODJBZ7%2F20250915%2Feu-south-1%2Fs3%2Faws4_request\\u0026X-Amz-Date=20250915T070517Z\\u0026X-Amz-Expires=3600\\u0026X-Amz-SignedHeaders=host\\u0026X-Amz-Security-Token=IQoJb3JpZ2luX2VjEPf%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCmV1LXNvdXRoLTEiRzBFAiEA%2Byp6ZLeUneu9101BeB66DK3TGGAyFJBnLH7B2aqSxHUCIHZc8%2BVntirdiVvPmQjvUxVUmUq0PsYEqvhfwPl1dX38KoUFCHAQAhoMMTk2MjQ0NDUyMDg1IgwkMqNYzLQtKgzo0csq4gQjgd8W4GHMretmUzzp5A1VX%2FqviCGtAqdo%2BP42%2FisY1pyYn8X%2B6kbBEv6wF3P39GmBmChyGytsr2mN5dyJa3lLaqiulPgHQVFy4u%2FbR1NhjsPeYITiXE9YF9hNXjjbuzYmgVExNAfHjLWItCaCYUOdNlWItIC88Jghiur6VxMAD0n7Uc8ZlUAaE7yFvk9iuFrcgQpHQ7JkCYgtzwV68J84%2FL%2BSjBnf22ZZqGKsocA0nI0W9xHgxd4a8xndwHu6lwsXcylXfaEcjLMoOP5%2BTmsIna0ES%2FCPcOWmm31oMqaHT9e5b0d2J%2FbIZTvqZV7l5kHxmIdvQ89xBEZ7OnMGMsTVmxfnSMS7MN7kBtTvpi73CIr4NXHVwa6qbCcvuKfyNlqgiKrsuTt1%2Bhg53%2FUwixxt6bciWpqI%2FItEHNyX%2FW9df7jG7CVI4h%2BAj7zcTmreWkmeMFQ46BJseC5AX%2FoLc2pMX9k%2BuZjNtGin3Qde0ngF4q54l%2Bow78Epw9lLjmk5jmmaXCk5iCw1OG3FUQVwkxzqSc5QTBgSJFeuyTuS8niUG92dknSHxpbnzQYGnbjo2XQFnnsnAGt91am7et9TQpyb4%2FpBdoiuu%2FFb7jFNmgnrQwoTfuZClr7PkJzqaVqj3UeAMIypqY%2Fwt8C8AGUKrPgMlhYZXKLyGuX1Q5aCTwRHcf1A4TCfYrsrI%2BRrzm4JbU%2FQfrOeaaFCAwzj5bGkWAZIWXhefu6FrYuV2KWWxjhpP1%2F%2BDuw9fCaH9iZJWoXeCLoEOmZr83FywCsZITZU78qdYfI4o0wL%2Fi2DtyrZwv1PaFq8MP7znsYGOpoBkcY9ln244b5OEVUG%2BJKUZ%2BEukm0MstvVYb4Udx49RQbZ%2FxYUVU0uSaASThwSoAlsJF276DdWkYfeg0i9sB%2FzkSDYXyyZjFIgTeGKUBbTbxNNZWOY%2Bltaf7GdP4mgKdCbp6glhJdp2cg2wgYAwp2nBFzgl0VOMHFSXzZw58cYlSs9hRJqTIyPdV74RSw8RViP6bEh8WASfbxxhg%3D%3D\\u0026X-Amz-Signature=7e5ed3065dad6db2158b63db47c288803378840d25d6bf047c61ab3aa8130055\",\"fullName\":\"Octopus Flex Mono\",\"description\":\"La tariffa Octopus Flex prevede un prezzo per la materia energia indicizzato, ovvero sempre in linea con il mercato all’ingrosso. I costi di commercializzazione sono invece bloccati per sempre, per una bolletta senza sorprese.\",\"params\":{\"productType\":\"VARIABLE\",\"annualStandingCharge\":\"84\",\"consumptionCharge\":\"0,0088\"}},{\"__typename\":\"GasProductType\",\"code\":\"000129GSVML11XXXXXXXOCTOFLEXGv11\",\"displayName\":\"Octopus Flex Gas\",\"termsAndConditionsUrl\":\"https://s3.eu-south-1.amazonaws.com/oeit-prod-user-documents/terms-and-conditions/Octo%20Flex%20Gas%20Domestica-11-1.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256\\u0026X-Amz-Credential=ASIAS3MIUWL27HODJBZ7%2F20250915%2Feu-south-1%2Fs3%2Faws4_request\\u0026X-Amz-Date=20250915T070517Z\\u0026X-Amz-Expires=86400\\u0026X-Amz-SignedHeaders=host\\u0026X-Amz-Security-Token=IQoJb3JpZ2luX2VjEPf%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCmV1LXNvdXRoLTEiRzBFAiEA%2Byp6ZLeUneu9101BeB66DK3TGGAyFJBnLH7B2aqSxHUCIHZc8%2BVntirdiVvPmQjvUxVUmUq0PsYEqvhfwPl1dX38KoUFCHAQAhoMMTk2MjQ0NDUyMDg1IgwkMqNYzLQtKgzo0csq4gQjgd8W4GHMretmUzzp5A1VX%2FqviCGtAqdo%2BP42%2FisY1pyYn8X%2B6kbBEv6wF3P39GmBmChyGytsr2mN5dyJa3lLaqiulPgHQVFy4u%2FbR1NhjsPeYITiXE9YF9hNXjjbuzYmgVExNAfHjLWItCaCYUOdNlWItIC88Jghiur6VxMAD0n7Uc8ZlUAaE7yFvk9iuFrcgQpHQ7JkCYgtzwV68J84%2FL%2BSjBnf22ZZqGKsocA0nI0W9xHgxd4a8xndwHu6lwsXcylXfaEcjLMoOP5%2BTmsIna0ES%2FCPcOWmm31oMqaHT9e5b0d2J%2FbIZTvqZV7l5kHxmIdvQ89xBEZ7OnMGMsTVmxfnSMS7MN7kBtTvpi73CIr4NXHVwa6qbCcvuKfyNlqgiKrsuTt1%2Bhg53%2FUwixxt6bciWpqI%2FItEHNyX%2FW9df7jG7CVI4h%2BAj7zcTmreWkmeMFQ46BJseC5AX%2FoLc2pMX9k%2BuZjNtGin3Qde0ngF4q54l%2Bow78Epw9lLjmk5jmmaXCk5iCw1OG3FUQVwkxzqSc5QTBgSJFeuyTuS8niUG92dknSHxpbnzQYGnbjo2XQFnnsnAGt91am7et9TQpyb4%2FpBdoiuu%2FFb7jFNmgnrQwoTfuZClr7PkJzqaVqj3UeAMIypqY%2Fwt8C8AGUKrPgMlhYZXKLyGuX1Q5aCTwRHcf1A4TCfYrsrI%2BRrzm4JbU%2FQfrOeaaFCAwzj5bGkWAZIWXhefu6FrYuV2KWWxjhpP1%2F%2BDuw9fCaH9iZJWoXeCLoEOmZr83FywCsZITZU78qdYfI4o0wL%2Fi2DtyrZwv1PaFq8MP7znsYGOpoBkcY9ln244b5OEVUG%2BJKUZ%2BEukm0MstvVYb4Udx49RQbZ%2FxYUVU0uSaASThwSoAlsJF276DdWkYfeg0i9sB%2FzkSDYXyyZjFIgTeGKUBbTbxNNZWOY%2Bltaf7GdP4mgKdCbp6glhJdp2cg2wgYAwp2nBFzgl0VOMHFSXzZw58cYlSs9hRJqTIyPdV74RSw8RViP6bEh8WASfbxxhg%3D%3D\\u0026X-Amz-Signature=d707d547c7dc9bd28d77a5910b6add8cbed37a57a653837838f7bcc14e1ab473\",\"fullName\":\"Octopus Flex Gas\",\"description\":\"La tariffa Octopus Flex prevede un prezzo per la materia energia indicizzato, ovvero sempre in linea con il mercato all’ingrosso. I costi di commercializzazione sono invece bloccati per sempre, per una bolletta senza sorprese.\",\"params\":{\"productType\":\"VARIABLE\",\"annualStandingCharge\":\"84\",\"consumptionCharge\":\"0,08\"}},{\"__typename\":\"GasProductType\",\"code\":\"000129GSFML28XXXXXXXXOCTOFIXGv28\",\"displayName\":\"Octopus Fissa 12M Gas\",\"termsAndConditionsUrl\":\"https://s3.eu-south-1.amazonaws.com/oeit-prod-user-documents/terms-and-conditions/Octo%20Fissa%2012M%20Gas%20Domestico-28-1.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256\\u0026X-Amz-Credential=ASIAS3MIUWL27HODJBZ7%2F20250915%2Feu-south-1%2Fs3%2Faws4_request\\u0026X-Amz-Date=20250915T070517Z\\u0026X-Amz-Expires=86400\\u0026X-Amz-SignedHeaders=host\\u0026X-Amz-Security-Token=IQoJb3JpZ2luX2VjEPf%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCmV1LXNvdXRoLTEiRzBFAiEA%2Byp6ZLeUneu9101BeB66DK3TGGAyFJBnLH7B2aqSxHUCIHZc8%2BVntirdiVvPmQjvUxVUmUq0PsYEqvhfwPl1dX38KoUFCHAQAhoMMTk2MjQ0NDUyMDg1IgwkMqNYzLQtKgzo0csq4gQjgd8W4GHMretmUzzp5A1VX%2FqviCGtAqdo%2BP42%2FisY1pyYn8X%2B6kbBEv6wF3P39GmBmChyGytsr2mN5dyJa3lLaqiulPgHQVFy4u%2FbR1NhjsPeYITiXE9YF9hNXjjbuzYmgVExNAfHjLWItCaCYUOdNlWItIC88Jghiur6VxMAD0n7Uc8ZlUAaE7yFvk9iuFrcgQpHQ7JkCYgtzwV68J84%2FL%2BSjBnf22ZZqGKsocA0nI0W9xHgxd4a8xndwHu6lwsXcylXfaEcjLMoOP5%2BTmsIna0ES%2FCPcOWmm31oMqaHT9e5b0d2J%2FbIZTvqZV7l5kHxmIdvQ89xBEZ7OnMGMsTVmxfnSMS7MN7kBtTvpi73CIr4NXHVwa6qbCcvuKfyNlqgiKrsuTt1%2Bhg53%2FUwixxt6bciWpqI%2FItEHNyX%2FW9df7jG7CVI4h%2BAj7zcTmreWkmeMFQ46BJseC5AX%2FoLc2pMX9k%2BuZjNtGin3Qde0ngF4q54l%2Bow78Epw9lLjmk5jmmaXCk5iCw1OG3FUQVwkxzqSc5QTBgSJFeuyTuS8niUG92dknSHxpbnzQYGnbjo2XQFnnsnAGt91am7et9TQpyb4%2FpBdoiuu%2FFb7jFNmgnrQwoTfuZClr7PkJzqaVqj3UeAMIypqY%2Fwt8C8AGUKrPgMlhYZXKLyGuX1Q5aCTwRHcf1A4TCfYrsrI%2BRrzm4JbU%2FQfrOeaaFCAwzj5bGkWAZIWXhefu6FrYuV2KWWxjhpP1%2F%2BDuw9fCaH9iZJWoXeCLoEOmZr83FywCsZITZU78qdYfI4o0wL%2Fi2DtyrZwv1PaFq8MP7znsYGOpoBkcY9ln244b5OEVUG%2BJKUZ%2BEukm0MstvVYb4Udx49RQbZ%2FxYUVU0uSaASThwSoAlsJF276DdWkYfeg0i9sB%2FzkSDYXyyZjFIgTeGKUBbTbxNNZWOY%2Bltaf7GdP4mgKdCbp6glhJdp2cg2wgYAwp2nBFzgl0VOMHFSXzZw58cYlSs9hRJqTIyPdV74RSw8RViP6bEh8WASfbxxhg%3D%3D\\u0026X-Amz-Signature=c40138ee98f34021289a34cb26d7be584afd7cd9e8d65ebc123bff2aa8fd66ea\",\"fullName\":\"Octopus Fissa 12M Gas\",\"description\":\"Con Octopus Fissa 12M, blocchi il prezzo della materia energia e dei costi di commercializzazione per un anno. Al termine dei 12 mesi potrai scegliere nuovamente la tariffa più conveniente per te (Fissa o Flex) e continuare a risparmiare, senza stress.\",\"params\":{\"productType\":\"FIXED_SINGLE_RATE\",\"annualStandingCharge\":\"84\",\"consumptionCharge\":\"0,4295\"}}]},\"__N_SSG\":true},\"page\":\"/le-nostre-tariffe\",\"query\":{},\"buildId\":\"NlQ1Aep3RU_XERVlZBCLG\",\"isFallback\":false,\"isExperimentalCompile\":false,\"gsp\":true,\"locale\":\"it\",\"locales\":[\"it\"],\"defaultLocale\":\"it\",\"scriptLoader\":[]}"
```
- Grab the content of the field `props.pageProps`, which will be an array of objects.
Here is an example:
```
[
      {
        "__typename": "ElectricityProductType",
        "code": "000129ESFML08XXXXXXXXOCTOFIXv108",
        "displayName": "Octopus Fissa 12M",
        "termsAndConditionsUrl": "https://s3.eu-south-1.amazonaws.com/oeit-prod-user-documents/terms-and-conditions/Octo%20Fissa%2012M%20EE%20Domestico-108-1.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ASIAS3MIUWL27HODJBZ7%2F20250915%2Feu-south-1%2Fs3%2Faws4_request&X-Amz-Date=20250915T070517Z&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&X-Amz-Security-Token=IQoJb3JpZ2luX2VjEPf%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCmV1LXNvdXRoLTEiRzBFAiEA%2Byp6ZLeUneu9101BeB66DK3TGGAyFJBnLH7B2aqSxHUCIHZc8%2BVntirdiVvPmQjvUxVUmUq0PsYEqvhfwPl1dX38KoUFCHAQAhoMMTk2MjQ0NDUyMDg1IgwkMqNYzLQtKgzo0csq4gQjgd8W4GHMretmUzzp5A1VX%2FqviCGtAqdo%2BP42%2FisY1pyYn8X%2B6kbBEv6wF3P39GmBmChyGytsr2mN5dyJa3lLaqiulPgHQVFy4u%2FbR1NhjsPeYITiXE9YF9hNXjjbuzYmgVExNAfHjLWItCaCYUOdNlWItIC88Jghiur6VxMAD0n7Uc8ZlUAaE7yFvk9iuFrcgQpHQ7JkCYgtzwV68J84%2FL%2BSjBnf22ZZqGKsocA0nI0W9xHgxd4a8xndwHu6lwsXcylXfaEcjLMoOP5%2BTmsIna0ES%2FCPcOWmm31oMqaHT9e5b0d2J%2FbIZTvqZV7l5kHxmIdvQ89xBEZ7OnMGMsTVmxfnSMS7MN7kBtTvpi73CIr4NXHVwa6qbCcvuKfyNlqgiKrsuTt1%2Bhg53%2FUwixxt6bciWpqI%2FItEHNyX%2FW9df7jG7CVI4h%2BAj7zcTmreWkmeMFQ46BJseC5AX%2FoLc2pMX9k%2BuZjNtGin3Qde0ngF4q54l%2Bow78Epw9lLjmk5jmmaXCk5iCw1OG3FUQVwkxzqSc5QTBgSJFeuyTuS8niUG92dknSHxpbnzQYGnbjo2XQFnnsnAGt91am7et9TQpyb4%2FpBdoiuu%2FFb7jFNmgnrQwoTfuZClr7PkJzqaVqj3UeAMIypqY%2Fwt8C8AGUKrPgMlhYZXKLyGuX1Q5aCTwRHcf1A4TCfYrsrI%2BRrzm4JbU%2FQfrOeaaFCAwzj5bGkWAZIWXhefu6FrYuV2KWWxjhpP1%2F%2BDuw9fCaH9iZJWoXeCLoEOmZr83FywCsZITZU78qdYfI4o0wL%2Fi2DtyrZwv1PaFq8MP7znsYGOpoBkcY9ln244b5OEVUG%2BJKUZ%2BEukm0MstvVYb4Udx49RQbZ%2FxYUVU0uSaASThwSoAlsJF276DdWkYfeg0i9sB%2FzkSDYXyyZjFIgTeGKUBbTbxNNZWOY%2Bltaf7GdP4mgKdCbp6glhJdp2cg2wgYAwp2nBFzgl0VOMHFSXzZw58cYlSs9hRJqTIyPdV74RSw8RViP6bEh8WASfbxxhg%3D%3D&X-Amz-Signature=69d244c4115a55d6825e13b46b56ad0b9c27793a1efc7193323ede7512fe59fe",
        "fullName": "Octopus Fissa 12M",
        "description": "Con Octopus Fissa 12M, blocchi il prezzo della materia energia e dei costi di commercializzazione per un anno. Al termine dei 12 mesi potrai scegliere nuovamente la tariffa più conveniente per te (Fissa o Flex) e continuare a risparmiare, senza stress.",
        "params": {
          "productType": "FIXED_SINGLE_RATE",
          "annualStandingCharge": "84",
          "consumptionCharge": "0,1089"
        }
      },
      {
        "__typename": "ElectricityProductType",
        "code": "000129ESVFL89XXXXXXXXOCTOFLEXv89",
        "displayName": "Octopus Flex",
        "termsAndConditionsUrl": "https://s3.eu-south-1.amazonaws.com/oeit-prod-user-documents/terms-and-conditions/Octo%20Flex%20EE%20Domestico-89-1.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ASIAS3MIUWL27HODJBZ7%2F20250915%2Feu-south-1%2Fs3%2Faws4_request&X-Amz-Date=20250915T070517Z&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&X-Amz-Security-Token=IQoJb3JpZ2luX2VjEPf%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCmV1LXNvdXRoLTEiRzBFAiEA%2Byp6ZLeUneu9101BeB66DK3TGGAyFJBnLH7B2aqSxHUCIHZc8%2BVntirdiVvPmQjvUxVUmUq0PsYEqvhfwPl1dX38KoUFCHAQAhoMMTk2MjQ0NDUyMDg1IgwkMqNYzLQtKgzo0csq4gQjgd8W4GHMretmUzzp5A1VX%2FqviCGtAqdo%2BP42%2FisY1pyYn8X%2B6kbBEv6wF3P39GmBmChyGytsr2mN5dyJa3lLaqiulPgHQVFy4u%2FbR1NhjsPeYITiXE9YF9hNXjjbuzYmgVExNAfHjLWItCaCYUOdNlWItIC88Jghiur6VxMAD0n7Uc8ZlUAaE7yFvk9iuFrcgQpHQ7JkCYgtzwV68J84%2FL%2BSjBnf22ZZqGKsocA0nI0W9xHgxd4a8xndwHu6lwsXcylXfaEcjLMoOP5%2BTmsIna0ES%2FCPcOWmm31oMqaHT9e5b0d2J%2FbIZTvqZV7l5kHxmIdvQ89xBEZ7OnMGMsTVmxfnSMS7MN7kBtTvpi73CIr4NXHVwa6qbCcvuKfyNlqgiKrsuTt1%2Bhg53%2FUwixxt6bciWpqI%2FItEHNyX%2FW9df7jG7CVI4h%2BAj7zcTmreWkmeMFQ46BJseC5AX%2FoLc2pMX9k%2BuZjNtGin3Qde0ngF4q54l%2Bow78Epw9lLjmk5jmmaXCk5iCw1OG3FUQVwkxzqSc5QTBgSJFeuyTuS8niUG92dknSHxpbnzQYGnbjo2XQFnnsnAGt91am7et9TQpyb4%2FpBdoiuu%2FFb7jFNmgnrQwoTfuZClr7PkJzqaVqj3UeAMIypqY%2Fwt8C8AGUKrPgMlhYZXKLyGuX1Q5aCTwRHcf1A4TCfYrsrI%2BRrzm4JbU%2FQfrOeaaFCAwzj5bGkWAZIWXhefu6FrYuV2KWWxjhpP1%2F%2BDuw9fCaH9iZJWoXeCLoEOmZr83FywCsZITZU78qdYfI4o0wL%2Fi2DtyrZwv1PaFq8MP7znsYGOpoBkcY9ln244b5OEVUG%2BJKUZ%2BEukm0MstvVYb4Udx49RQbZ%2FxYUVU0uSaASThwSoAlsJF276DdWkYfeg0i9sB%2FzkSDYXyyZjFIgTeGKUBbTbxNNZWOY%2Bltaf7GdP4mgKdCbp6glhJdp2cg2wgYAwp2nBFzgl0VOMHFSXzZw58cYlSs9hRJqTIyPdV74RSw8RViP6bEh8WASfbxxhg%3D%3D&X-Amz-Signature=6da7c180cfa6437d3895c69e2dcfe517a363cfb49a2e693cb9426e5d01f6f997",
        "fullName": "Octopus Flex Multi",
        "description": "La tariffa Octopus Flex prevede un prezzo per la materia energia indicizzato, ovvero sempre in linea con il mercato all’ingrosso. I costi di commercializzazione sono invece bloccati per sempre, per una bolletta senza sorprese.",
        "params": {
          "productType": "VARIABLE",
          "annualStandingCharge": "84",
          "consumptionCharge": "0,0088"
        }
      },
      {
        "__typename": "ElectricityProductType",
        "code": "000129ESVML75XXXXXOCTOFLEXMONv75",
        "displayName": "Octopus Flex Mono",
        "termsAndConditionsUrl": "https://s3.eu-south-1.amazonaws.com/oeit-prod-user-documents/terms-and-conditions/Octopus%20Flex%20Mono%20EE%20Domestico-75-1.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ASIAS3MIUWL27HODJBZ7%2F20250915%2Feu-south-1%2Fs3%2Faws4_request&X-Amz-Date=20250915T070517Z&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&X-Amz-Security-Token=IQoJb3JpZ2luX2VjEPf%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCmV1LXNvdXRoLTEiRzBFAiEA%2Byp6ZLeUneu9101BeB66DK3TGGAyFJBnLH7B2aqSxHUCIHZc8%2BVntirdiVvPmQjvUxVUmUq0PsYEqvhfwPl1dX38KoUFCHAQAhoMMTk2MjQ0NDUyMDg1IgwkMqNYzLQtKgzo0csq4gQjgd8W4GHMretmUzzp5A1VX%2FqviCGtAqdo%2BP42%2FisY1pyYn8X%2B6kbBEv6wF3P39GmBmChyGytsr2mN5dyJa3lLaqiulPgHQVFy4u%2FbR1NhjsPeYITiXE9YF9hNXjjbuzYmgVExNAfHjLWItCaCYUOdNlWItIC88Jghiur6VxMAD0n7Uc8ZlUAaE7yFvk9iuFrcgQpHQ7JkCYgtzwV68J84%2FL%2BSjBnf22ZZqGKsocA0nI0W9xHgxd4a8xndwHu6lwsXcylXfaEcjLMoOP5%2BTmsIna0ES%2FCPcOWmm31oMqaHT9e5b0d2J%2FbIZTvqZV7l5kHxmIdvQ89xBEZ7OnMGMsTVmxfnSMS7MN7kBtTvpi73CIr4NXHVwa6qbCcvuKfyNlqgiKrsuTt1%2Bhg53%2FUwixxt6bciWpqI%2FItEHNyX%2FW9df7jG7CVI4h%2BAj7zcTmreWkmeMFQ46BJseC5AX%2FoLc2pMX9k%2BuZjNtGin3Qde0ngF4q54l%2Bow78Epw9lLjmk5jmmaXCk5iCw1OG3FUQVwkxzqSc5QTBgSJFeuyTuS8niUG92dknSHxpbnzQYGnbjo2XQFnnsnAGt91am7et9TQpyb4%2FpBdoiuu%2FFb7jFNmgnrQwoTfuZClr7PkJzqaVqj3UeAMIypqY%2Fwt8C8AGUKrPgMlhYZXKLyGuX1Q5aCTwRHcf1A4TCfYrsrI%2BRrzm4JbU%2FQfrOeaaFCAwzj5bGkWAZIWXhefu6FrYuV2KWWxjhpP1%2F%2BDuw9fCaH9iZJWoXeCLoEOmZr83FywCsZITZU78qdYfI4o0wL%2Fi2DtyrZwv1PaFq8MP7znsYGOpoBkcY9ln244b5OEVUG%2BJKUZ%2BEukm0MstvVYb4Udx49RQbZ%2FxYUVU0uSaASThwSoAlsJF276DdWkYfeg0i9sB%2FzkSDYXyyZjFIgTeGKUBbTbxNNZWOY%2Bltaf7GdP4mgKdCbp6glhJdp2cg2wgYAwp2nBFzgl0VOMHFSXzZw58cYlSs9hRJqTIyPdV74RSw8RViP6bEh8WASfbxxhg%3D%3D&X-Amz-Signature=7e5ed3065dad6db2158b63db47c288803378840d25d6bf047c61ab3aa8130055",
        "fullName": "Octopus Flex Mono",
        "description": "La tariffa Octopus Flex prevede un prezzo per la materia energia indicizzato, ovvero sempre in linea con il mercato all’ingrosso. I costi di commercializzazione sono invece bloccati per sempre, per una bolletta senza sorprese.",
        "params": {
          "productType": "VARIABLE",
          "annualStandingCharge": "84",
          "consumptionCharge": "0,0088"
        }
      },
      {
        "__typename": "GasProductType",
        "code": "000129GSVML11XXXXXXXOCTOFLEXGv11",
        "displayName": "Octopus Flex Gas",
        "termsAndConditionsUrl": "https://s3.eu-south-1.amazonaws.com/oeit-prod-user-documents/terms-and-conditions/Octo%20Flex%20Gas%20Domestica-11-1.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ASIAS3MIUWL27HODJBZ7%2F20250915%2Feu-south-1%2Fs3%2Faws4_request&X-Amz-Date=20250915T070517Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Security-Token=IQoJb3JpZ2luX2VjEPf%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCmV1LXNvdXRoLTEiRzBFAiEA%2Byp6ZLeUneu9101BeB66DK3TGGAyFJBnLH7B2aqSxHUCIHZc8%2BVntirdiVvPmQjvUxVUmUq0PsYEqvhfwPl1dX38KoUFCHAQAhoMMTk2MjQ0NDUyMDg1IgwkMqNYzLQtKgzo0csq4gQjgd8W4GHMretmUzzp5A1VX%2FqviCGtAqdo%2BP42%2FisY1pyYn8X%2B6kbBEv6wF3P39GmBmChyGytsr2mN5dyJa3lLaqiulPgHQVFy4u%2FbR1NhjsPeYITiXE9YF9hNXjjbuzYmgVExNAfHjLWItCaCYUOdNlWItIC88Jghiur6VxMAD0n7Uc8ZlUAaE7yFvk9iuFrcgQpHQ7JkCYgtzwV68J84%2FL%2BSjBnf22ZZqGKsocA0nI0W9xHgxd4a8xndwHu6lwsXcylXfaEcjLMoOP5%2BTmsIna0ES%2FCPcOWmm31oMqaHT9e5b0d2J%2FbIZTvqZV7l5kHxmIdvQ89xBEZ7OnMGMsTVmxfnSMS7MN7kBtTvpi73CIr4NXHVwa6qbCcvuKfyNlqgiKrsuTt1%2Bhg53%2FUwixxt6bciWpqI%2FItEHNyX%2FW9df7jG7CVI4h%2BAj7zcTmreWkmeMFQ46BJseC5AX%2FoLc2pMX9k%2BuZjNtGin3Qde0ngF4q54l%2Bow78Epw9lLjmk5jmmaXCk5iCw1OG3FUQVwkxzqSc5QTBgSJFeuyTuS8niUG92dknSHxpbnzQYGnbjo2XQFnnsnAGt91am7et9TQpyb4%2FpBdoiuu%2FFb7jFNmgnrQwoTfuZClr7PkJzqaVqj3UeAMIypqY%2Fwt8C8AGUKrPgMlhYZXKLyGuX1Q5aCTwRHcf1A4TCfYrsrI%2BRrzm4JbU%2FQfrOeaaFCAwzj5bGkWAZIWXhefu6FrYuV2KWWxjhpP1%2F%2BDuw9fCaH9iZJWoXeCLoEOmZr83FywCsZITZU78qdYfI4o0wL%2Fi2DtyrZwv1PaFq8MP7znsYGOpoBkcY9ln244b5OEVUG%2BJKUZ%2BEukm0MstvVYb4Udx49RQbZ%2FxYUVU0uSaASThwSoAlsJF276DdWkYfeg0i9sB%2FzkSDYXyyZjFIgTeGKUBbTbxNNZWOY%2Bltaf7GdP4mgKdCbp6glhJdp2cg2wgYAwp2nBFzgl0VOMHFSXzZw58cYlSs9hRJqTIyPdV74RSw8RViP6bEh8WASfbxxhg%3D%3D&X-Amz-Signature=d707d547c7dc9bd28d77a5910b6add8cbed37a57a653837838f7bcc14e1ab473",
        "fullName": "Octopus Flex Gas",
        "description": "La tariffa Octopus Flex prevede un prezzo per la materia energia indicizzato, ovvero sempre in linea con il mercato all’ingrosso. I costi di commercializzazione sono invece bloccati per sempre, per una bolletta senza sorprese.",
        "params": {
          "productType": "VARIABLE",
          "annualStandingCharge": "84",
          "consumptionCharge": "0,08"
        }
      },
      {
        "__typename": "GasProductType",
        "code": "000129GSFML28XXXXXXXXOCTOFIXGv28",
        "displayName": "Octopus Fissa 12M Gas",
        "termsAndConditionsUrl": "https://s3.eu-south-1.amazonaws.com/oeit-prod-user-documents/terms-and-conditions/Octo%20Fissa%2012M%20Gas%20Domestico-28-1.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ASIAS3MIUWL27HODJBZ7%2F20250915%2Feu-south-1%2Fs3%2Faws4_request&X-Amz-Date=20250915T070517Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Security-Token=IQoJb3JpZ2luX2VjEPf%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCmV1LXNvdXRoLTEiRzBFAiEA%2Byp6ZLeUneu9101BeB66DK3TGGAyFJBnLH7B2aqSxHUCIHZc8%2BVntirdiVvPmQjvUxVUmUq0PsYEqvhfwPl1dX38KoUFCHAQAhoMMTk2MjQ0NDUyMDg1IgwkMqNYzLQtKgzo0csq4gQjgd8W4GHMretmUzzp5A1VX%2FqviCGtAqdo%2BP42%2FisY1pyYn8X%2B6kbBEv6wF3P39GmBmChyGytsr2mN5dyJa3lLaqiulPgHQVFy4u%2FbR1NhjsPeYITiXE9YF9hNXjjbuzYmgVExNAfHjLWItCaCYUOdNlWItIC88Jghiur6VxMAD0n7Uc8ZlUAaE7yFvk9iuFrcgQpHQ7JkCYgtzwV68J84%2FL%2BSjBnf22ZZqGKsocA0nI0W9xHgxd4a8xndwHu6lwsXcylXfaEcjLMoOP5%2BTmsIna0ES%2FCPcOWmm31oMqaHT9e5b0d2J%2FbIZTvqZV7l5kHxmIdvQ89xBEZ7OnMGMsTVmxfnSMS7MN7kBtTvpi73CIr4NXHVwa6qbCcvuKfyNlqgiKrsuTt1%2Bhg53%2FUwixxt6bciWpqI%2FItEHNyX%2FW9df7jG7CVI4h%2BAj7zcTmreWkmeMFQ46BJseC5AX%2FoLc2pMX9k%2BuZjNtGin3Qde0ngF4q54l%2Bow78Epw9lLjmk5jmmaXCk5iCw1OG3FUQVwkxzqSc5QTBgSJFeuyTuS8niUG92dknSHxpbnzQYGnbjo2XQFnnsnAGt91am7et9TQpyb4%2FpBdoiuu%2FFb7jFNmgnrQwoTfuZClr7PkJzqaVqj3UeAMIypqY%2Fwt8C8AGUKrPgMlhYZXKLyGuX1Q5aCTwRHcf1A4TCfYrsrI%2BRrzm4JbU%2FQfrOeaaFCAwzj5bGkWAZIWXhefu6FrYuV2KWWxjhpP1%2F%2BDuw9fCaH9iZJWoXeCLoEOmZr83FywCsZITZU78qdYfI4o0wL%2Fi2DtyrZwv1PaFq8MP7znsYGOpoBkcY9ln244b5OEVUG%2BJKUZ%2BEukm0MstvVYb4Udx49RQbZ%2FxYUVU0uSaASThwSoAlsJF276DdWkYfeg0i9sB%2FzkSDYXyyZjFIgTeGKUBbTbxNNZWOY%2Bltaf7GdP4mgKdCbp6glhJdp2cg2wgYAwp2nBFzgl0VOMHFSXzZw58cYlSs9hRJqTIyPdV74RSw8RViP6bEh8WASfbxxhg%3D%3D&X-Amz-Signature=c40138ee98f34021289a34cb26d7be584afd7cd9e8d65ebc123bff2aa8fd66ea",
        "fullName": "Octopus Fissa 12M Gas",
        "description": "Con Octopus Fissa 12M, blocchi il prezzo della materia energia e dei costi di commercializzazione per un anno. Al termine dei 12 mesi potrai scegliere nuovamente la tariffa più conveniente per te (Fissa o Flex) e continuare a risparmiare, senza stress.",
        "params": {
          "productType": "FIXED_SINGLE_RATE",
          "annualStandingCharge": "84",
          "consumptionCharge": "0,4295"
        }
      }
    ]
```
- Filter thos items that have `params.productType` equal to `FIXED_SINGLE_RATE`.
- Now it's time to login into Octopus' account. Invoke the endpoint `https://octopusenergy.it/api/auth/login` using:
  -  `POST` method
  -  the `content-type` header configured to `text/plain;charset=UTF-8`
  -  the body must have 2 fields, `email` and `password` like: `{"email": "foo@email.com", "password": "fooPassword"}`. The values of `email` and `password` are retrieved through environment variables.
-  The endpoint's response will be something similar to the following. You must extract the value of the `accessToken` cookie.
```
[
  {
    "body": {
      "data": "Success"
    },
    "headers": {
      "cache-control": "no-cache, no-store, max-age=0, must-revalidate",
      "content-length": "18",
      "content-type": "application/json; charset=utf-8",
      "date": "Mon, 15 Sep 2025 07:41:18 GMT",
      "etag": "\"rbli7kuojyi\"",
      "server": "Vercel",
      "set-cookie": [
        "accessToken=; Max-Age=-1; Path=/",
        "refreshToken=; Max-Age=-1; Path=/",
        "masqueradeToken=; Max-Age=-1; Path=/",
        "scopedToken=; Max-Age=-1; Path=/",
        "sub=; Max-Age=-1; Path=/",
        "MWAuthToken=; Max-Age=-1; Path=/",
        "authProvider=; Max-Age=-1; Path=/",
        "accessToken=ACCESS_TOKEN_VALUE; Path=/; Expires=Mon, 15 Sep 2025 08:41:17 GMT; HttpOnly; Secure; SameSite=Strict",
        "refreshToken=5b081b6a33592591229f7de425ca5c2f903ad1169edbe0782eaae6a674763603; Path=/; Expires=Mon, 22 Sep 2025 07:41:17 GMT; HttpOnly; Secure; SameSite=Strict",
        "sub=kraken%7Caccount-user%3A516310; Path=/; Expires=Mon, 22 Sep 2025 07:41:17 GMT; HttpOnly; Secure; SameSite=Strict",
        "authProvider=email; Path=/; Expires=Mon, 22 Sep 2025 07:41:17 GMT; HttpOnly; Secure; SameSite=Strict"
      ],
      "strict-transport-security": "max-age=63072000",
      "x-matched-path": "/api/auth/login",
      "x-vercel-cache": "MISS",
      "x-vercel-id": "fra1::fra1::w6w6v-1757922077727-0f299a1c6b55",
      "connection": "close"
    },
    "statusCode": 200,
    "statusMessage": "OK"
  }
]
```
- Now it's time to retrieve the user's data, using the graphql endpoint `https://api.oeit-kraken.energy/v1/graphql/`. 
To work properly, you must set the `Authorization` header which must have the value of the `accessToken` extracted above.
The query must be:
```
query Viewer {
  viewer {
    email
    fullName
    accounts {
      ... on AccountType {
        number
        properties {
          electricitySupplyPoints {
            status
            product {
              displayName
              params {
                consumptionCharge
                annualStandingCharge
                productType
              }
            }
          }
          gasSupplyPoints {
            status
            product {
              params {
                consumptionCharge
                annualStandingCharge
                productType
              }
            }
          }
        }
      }
    }
  }
}
```
The response will be similar to:
```
{
  "data": {
    "viewer": {
      "email": "foo@email.com",
      "fullName": "foo faa",
      "accounts": [
        {
          "number": "A-XXXXXXXX",
          "properties": [
            {
              "electricitySupplyPoints": [
                {
                  "status": "ON_SUPPLY",
                  "product": {
                    "displayName": "Octopus Fissa 12M",
                    "params": {
                      "consumptionCharge": "0,1089",
                      "annualStandingCharge": "84",
                      "productType": "FIXED_SINGLE_RATE"
                    }
                  }
                }
              ],
              "gasSupplyPoints": []
            }
          ]
        }
      ]
    }
  }
}
```
- For each account, we must:
  - take into account only the supply points that have `status` equal to `ON_SUPPLY`;
  - compare the subscribed price with the one actually available. All the `electricitySupplyPoints` must be compared to the offers with `__typename` equal to `ElectricityProductType`, while the `gasSupplyPoints` must be compared to the offers with `__typename` equal to `GasProductType`.
- If the comparison finds that the new offer has a lower raw price (`consumptionCharge`) and a lower or equal marketing price (`annualStandingCharge`) than the one I have now, we must send the e-mail to change my offer.
- The e-mail will be sent using the SMTP protocol. The variables needed to configure it will be defined in the environment variables.
The e-mail will be sent to `ciao@octopusenergy.it` and will have the following subject:
`Richiesta adeguamento tariffa {{ TIPO_TARIFFA }} - account {{ ACCOUNT_NUMBER }}`
Where:
    - `TIPO_TARIFFA` is `luce` if we are chaning the price of an electricity supply point, `gas` otherwise;
    - `ACCOUNT_NUMBER` is the value of the field `accounts.number` for which I'm requesting the change.
The e-mail's body will be the following:
```
Buongiorno,
con la presente richiedo l'adeguamento della mia tariffa {{ TIPO_TARIFFA }} con quella attualmente in commercio, per l'account {{ ACCOUNT_NUMBER }}.
In dettaglio, vorrei passare dalla mia tariffa da {{ PREZZO_ATTUALE }} a quella da {{ NUOVO_PREZZO }}.

Cordiali saluti,
{{ NOME_COMPLETO }}
```
Where:
    - `TIPO_TARIFFA` is `luce` if we are chaning the price of an electricity supply point, `gas` otherwise;
    - `ACCOUNT_NUMBER` is the value of the field `accounts.number` for which I'm requesting the change;
    - `PREZZO_ATTUALE` is the actual price for raw material I'm paying for;
    - `NUOVO_PREZZO` is the new price for raw material I'll pay.


All the code you are going to produce must be stored in multiple modules, to keep it maintainable. Each module must be unit tested.
If you are using a library, you must use the latest version of it.

For the cache file, use a JSON file for which the destination path is set through an environment variable.