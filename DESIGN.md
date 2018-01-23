# Stormtide Design
Designing a rules engine for *Magic* is a difficult and daunting task. This document will try to lay out some of plans for the simplest possible rules engine implementation that can handle the entirety of the game's black-bordered cards.

For the time being, the most current documentation lives in the code comments. See [src/game.rs](src/game.rs) for a good starting point.

## Objects
**MCR 109.1**:
>  An *object* is an ability on the stack, a card, a copy of a card, a token, a spell, a permanent, or an emblem.

The most naive way to represent objects would be to jam all possible *characteristics* (109.3) into struct fields with each one being optional. This would probably end up being super tedious and error-prone, since our type system wouldn't be able to prevent invalid objects like creatures without power and toughness.

Characteristics of objects need to be accessible regardless of an object's location, even if those characteristics are not normally necessary. For example, check out [**Essence Backlash**](http://gatherer.wizards.com/Pages/Card/Details.aspx?multiverseid=290527):

> Counter target creature spell. Essence Backlash deals damage equal to that spell's power to its controller.

Despite *power* not normally being relevant for a spell on the stack, it's still a characteristic that's present on creature spells!

Objects often change type, like becoming creatures in the case of creature lands like [**Treetop Village**](http://gatherer.wizards.com/Pages/Card/Details.aspx?multiverseid=417453) or planeswalkers like [**Gideon, Ally of Zendikar**](http://gatherer.wizards.com/Pages/Card/Details.aspx?multiverseid=401897). This complicates characteristic tracking, since the existence of certain characteristics become tied to continuous effects!