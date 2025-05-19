# **Quotex Market**

Quotex Markets provide the main interface for trading on the Quotex Platoform ,it utilises a robust orderbook system to ensure smooth trading experience ,allowing traders to utilise market orders and Limit orders

## **Features**

+ ### **Derivatves** :

   > Quotex Markets is a derivatives perpetual market ,Long Positions (buying) or Short Positions (selling) are done in the quote currency,
   >
   > Quotex Markets utilise an inbuilt timer system that handles funding rate payment between longs and shorts,Spot prices is fetched from the [XRC Cansiter](https://internetcomputer.org/docs/references/system-canisters/xrc) at hourly intervals to handle funding payment

+ ### **Limit Orders**:

   > Quotex features a limit order book where orders can be placed at sepcific price points and executed at those price points.>
   > WHen a price point is croosed ,all orders at that price point is assumed to have been filled and are handled accordingly

+ ### **Cross Price Swaps**:

   > Quotex markets features a cross price swapping mechanism for executing market orders at the closest possible prices if it could not be filled completely at the expected  price
   >
   > Traders set the max price slippage for executing their market orders starting from the best current price,and if the order is not completely filled at that price the next best order is checked for that

## **Terminologies**

+ ### **Ticks** :

   > Ticks are price mechanism within Quotex Market ,Order are placed in the Order Book at different Ticks and when making a swap ,the orders are filled at those Ticks
   >
   > Ticks are derived from price with the simple relation <br>
   >
   > __Tick = Price * Price_Factor__
   > So a price of $1200.28 per asset with a price factor of 1000 is denoted in tick as __tick = 1200.08 * 1000__ 
   >
   > Ticks serve as price indicators as they can be visualised as the ratio of the base asset against the  price of the quote asset expressed in percentage i.e <br>
   >
   > __Tick = (Price of Base Asset / Price of Quote Asset) * 100%__
   >
   > The difference in magnitude between two neigbouring neighbouring ticks is 1 basis point (0.01%) by default

+ ### **Tick Spacing** :

   > Tick Spacing serve as a tradeoff between price precision and efficiency ,tick spacing is used to specify the difference in magnitude between two neighbouring ticks<br>
   > which is 1 basis point (0.01%) by default , tick spacing serves as a multiple in power powers of ten of the defaut value . The magnitude difference is
   >
   > __1 * (tick_spacing) basis point unit__
   >
   > So a tick spacing of 100 has a magnitude difference of 1 * 100 = 100 basis point(1%)<br>
   >
   > A large tick spacing leads to less price precision as prices can only be set on certain ticks but swap iterations is typically done with less looping while a smaller tick spacing allows for more price precision but swaps can be very expensive due to small iteration steps in the swap loop .
