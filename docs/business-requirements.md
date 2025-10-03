This documents contains the business requirements of this project.

The name of the project is Polipo, and it allows me to obtain the cheapest offer for electricity and gas from my operator: Octopus Energy.

Every offer has 2 parameters to take into account:
- the price of raw material, per unit. It is expressed in €/kWh for the electricity, while for the gas is expressed in €/Smc
- the marketing price, per year: It is the fixed price that I need to pay to the provider for each of my forniture.

The logical flow is:
- Grab the latest 12 month fixed offer from Octopus' website for both electricity and gas;
- Login with my Octopus' account;
- Check the price I'm actually paying, for both raw material and marketing price;
- If the marketing price is lower or equal than the one I'm actually paying and the raw material price is lower than the one I'm actually paying then;
    - Send an e-mail to Octopus to require offer adjustment;
    - otherwise skip it

Consider that usually the new price doesn't get applied immediately, so the requester price should be stored in a cache file to avoid sending the e-mail multiple times until it activates.