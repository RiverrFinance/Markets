# **Quotex Market**

Quotex Markets provide the main interface for trading on the Quotex Platoform ,it utilises a robust orderbook system to ensure smooth trading experience ,allowing traders to utilise market orders and Limit orders

## **Features**
+ ### **Derivatves** :
   >Quotex Markets is a derivatives perpetual market ,Long Positions (buying) or Short Positions (selling) are done in the quote currency,
   >
   >Quotex Markets utilise an inbuilt timer system that handles funding rate payment between longs and shorts,Spot prices is fetched from the [XRC Cansiter](https://internetcomputer.org/docs/references/system-canisters/xrc) at hourly intervals to handle funding payment
+ ### **Limit Orders**:
   >Quotex features a limit order book where orders can be placed at sepcific price points and executed at those price points.
   >
   > WHen a price point is croosed ,all orders at that price point is assumed to have been filled and are handled accordingly 

+ ### **Cross Price Swaps**:
   >Quotex markets features a cross price swapping mechanism for executing market orders at the closest possible prices if it could not be filled completely at the expected  price 
   >
   >Traders set the max price slippage for executing their market orders starting from the best current price,and if the order is not completely filled at that price the next best order is checked for that  
