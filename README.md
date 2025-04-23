# Baby Steps with Rust - UDP Datagram

Hey there! I'm Tim. I primarily work as a Ruby on Rails developer, but I've been excited by Rust for a long time. And I have dabbled. Oh, how I have dabbled. But recently I've felt like it was time to write some more substantial Rust code, so I came up with some babysteps...

Any this is the first babystep! .

One thing that attracts me to Rust is that it is a systems language that you can also comfortably write web apps in. So, I wanted to start with something systems-y, but very small. Something with bytes! This led me to chose a very simple networking protocol: UDP.

UDP is User Datagram Protocol, and was created as part of the Internet Protocol Suite.

Breaking down its name further we learn that:
- User - highlights that this protocol is for the User or Application layer
- Datagram - is the unit of information we're dealing with. Different networking protocols use different terminologies for their units, and *Datagram* denotes that this unit is self-contained and doesn't rely on previous message exchanges.
- Protocol - Prooooootocol.

Today UDP is used where speed and low latency is more important than every single piece of data being delivered. Think streaming media and online gaming. Applications where you can afford to drop a message if it means that you can keep latency low.

One of its designers , David P. Reed, had this to say:
> UDP was actually “designed” in 30 minutes on a blackboard when we decided pull the original TCP protocol apart into TCP and IP...

Heck yeah. Babysteps.

I really like his blog post "udp and me", link below.
[dpr » udp and me](https://web.archive.org/web/20180919085731/https://www.deepplum.com/blog-dpr/?page_id=6)

We can see in the original RFC for it there's not too much to it. Perfect for dipping our toes into modeling data in Rust.
[RFC 768: User Datagram Protocol](https://www.rfc-editor.org/rfc/rfc768)

This RFC really takes a turn.
- Source Port: two bytes, optional (port or 0's) - cool
- Destination Port: two bytes - same deal
- Length: totally expected - length in *octets* of this user datagram including header and data.
Cool piece of trivia, old networking RFC's use the word 'octets' because at the time not all bytes were 8 bits long 
slide: Coding horror

but then we have..
Checksum
> Checksum is the 16-bit one's complement of the one's complement sum of a pseudo header of information from the IP header, the UDP header, and the data,  padded  with zero octets  at the end (if necessary)  to  make a multiple of two octets.

Happily, the RFC also says that "an all 0 checksum value means that the transmitter generated no checksum  (for debugging or for higher level protocols that don't care)."

Um, going to say that's its another two byte long value, though perhaps I will expand later, because it is pretty cool... The result will be all one's if if the checksum is correct!
Slide: "Feeling cute, might implement later"

Then we have
Payload:  Data octets. The medium, the message. We get to decide what they mean! 
Slide: Captain Planet "The power is yours!"

Anyway.

## Data Modeling

To describe the top level element here we will use a struct called 'UdpDatagram'.

What we're going to do is parse a collection of bytes into a collection of (hopefully) well-modeled fields.

A **struct** (short for structure) is a way to keep related data under the same roof. A named collection of fields, which can have thier own type.
They come in a couple different flavors, but the main one we are going to use is very similar to the struct in C and a bunch of other languages.

The fields for this are pretty clear, they're the ones defined in the RFC.

But - what types should they have?

I think Port should be a type in itself. Here we can use another kind of struct called a 'tuple struct'. I can imagine Port accumulating behavior, but we all know that under the hood it is really a 16 bit integer.

Length can be a 16 bit number, it make sense to me that it is a basic integer type, I don't think it can accumulate any behavior outside of the behavior of a number.

The checksum will be treated as bits, so we want to leave that as raw as possible.

Then the Payload itself feels a bit tricky.
It can be anything - it's on the user of the protocol to define what it means.

I don't have a concrete use case right now, so it feels right to have it be a Vector of u8's and consumers of this datagram could then write the conversion for whatever they need. They are unlikely to need any of the other parts of the datagram (in fact if we were using recv_from from either the Rust Standard Library or the system call itself we would only get back the length and the source address)
Link: [UdpSocket in std::net - Rust](https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.recv_from)

All right, now that we have modeled our data we can work on the conversion!
## Conversion

It turns out there is an idiomatic way to do conversions in Rust, and that is by using specific traits.

Traits are how shared behavior is defined across different concrete types.

To use a bunch of long words: It is polymorphism without inheritance.

I guess if it looks like a duck and quacks like a duck you don't care if it was descended from the great Asteriornis (aka the "Wonderchicken").
slide: Asteriornis

The Rust standard library has two traits for conversion: 
**From / Into** for infallible, can't miss conversions
**TryFrom / TryInto** for conversions that may fail

We'll use TryFrom because UDP Datagrams can become corrupted in transit and we want to signal to other developers calling this code that this conversion will return a Result enum, so they will have to handle a potential error.

By implementing the 'TryFrom' trait we will get some stuff for free.

Not only do we get the method we're looking for to go from bytes to datagram but we also get `try_into()` to call on bytes.

```
// Using TryFrom (destination-focused)
let datagram = UdpDatagram::try_from(&bytes[..])?; 
// Using TryInto (source-focused) 
let datagram: UdpDatagram = bytes.try_into()?;
```

Holy consistency Batman, I love anything that makes code more uniform (and saves me having to come up with a method name).

Apparently we will also be able to send our custom UDPDatagram to functions that say that they only accept items with the TryFrom trait bounds, which is much more general than whatever custom thing I was going to come up with.

The conversion code itself is pretty simple.

We know we have a Vector of bytes, the protocol tells us the order they are in, we just need to convert them into integers (and wrap them into their type in the case of Port)

`from_be_byte` stands for from **big endian** bytes
Big endian is when the most significant byte is on the left, the same that we write numbers.

For all standard internet protocols, the bytes are sent in big endian, in fact this is also called 'network byte order' - which is the opposite of what most computer CPU's utilize which is 'little endian byte order' where the least significant digit comes first.

As always we sit atop of bytes, and we get to decide what they mean. Some of this is for performance (you can do addition faster with little endian) but some is historical happenstance.

But I digress.

Then at the very end we take the rest of the bytes as their own collection we'll call the payload.
Seasoned Rust enthusiasts will notice that we copied data here. 
If we were super slick we wouldn't make a new copy of this data. But I am not slick, and doing so would lead us to worry about lifetimes, which is something I am not interested in doing just yet.

Sidebar: 
This is one of my long term learning tips: the farther you are outside of your domain the simpler you should make things. 
If I wanted to avoid creating a new Vec here I would shortchange some other learning (or dawdle too long on this program) and I doubt it is worth it. 
By all means play 'the full game' (which in this case means knowing that lifetimes exist and having a fuzzy idea what they are about) BUT know when there is enough on your plate. 
I am better knowing all the other concepts that are in the air here than getting a (poor) understanding of yet another one.

And there we have it! We can go from bytes to UDP Datagrams.

In the description there is a link to the program I wrote with tests. I would recommend deleting my TryFrom implementation and writing your own driven by these tests! Once you complete the implementation you can `cargo run` for a secret message!
