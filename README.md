# Vision

## Foreword

Vision is an office management solution, improving the vacation planning process. Its main goal was to focus on the algorithm, calculations and functionality, while leaving aside complex user behavior.

## Domain

The domain is based on 4 basic entities:

- vacation
- user - a person taking a vacation
- organization - a group of people who share some information among themselves, as well as the administrator of these people who can edit the states of their and their vacations
- transition - the change of the someone's work time

## Bank account

In the simplest model, the most important function of this type of service is to store the user's account balance - unused days-off, that they can exchange for vacation. In a sense, this is very similar to a bank account balance, to which we can add or subtract something. However, storing this information in the state, while simple and fulfilling, is not ideal in every way. Often a better system is to store a list of transactions, and when we need to check the state of the account, each time add them all up. This method is implemented by Vision. This not only provides a single source of information, but also protects against calculation errors. If any occur, once the algorithm is polished, all data will be immediately compatible with it. This also allows for later expansion of the system with additional functionality, all the while guaranteeing immediate compatibility with previous data.
Example: We add vacations in an organization, after some time we change the settings from the default generated 1.75 days-off per month for a full-time employee, to 1.5 days. The next time we perform an account calculation, we will include this condition is all previous vacations/transitions.

## Accesses

There are 3 types of information visibility in Vision:

- Admin level - data / activities permitted only to the person managing the organization.
- Organization level - data publicly available to anyone belonging to the organization
- User level - data available only to a given user

These 3 levels are implemented with 2 levels of access: administrator and user. The administrator has both administrator and organization, and the user has both user and organization visibility access level. In general, the administrator can see all data, the user level is mainly restricted to information such as "my vacations" - administrator is not a user and can not have vacations assigned, so they can't access this data.

Note: If an employee managing an organization wants to take a vacation, they should login as a user - it is possible to be logged simultaneously as an organization (administrator) and a user from one session (computer).

## Actions

### Adding vacation

Privilege level: Administrator
Add a vacation to a particular user belonging to an organization.

### Adding transition

Privilege level: Administrator
Adding transition to the given user belonging to the organization,

### Adding user

Privilege level: Administrator
Adding vacation to the given user belonging to the organization.

### Adding transition

Privilege level: Administrator
Adding transition to the given user belonging to the organization.

### Adding a user.

Privilege level: Administrator
Creating a new user belonging to the organization. In addition, it is required to add at least one transition (the first transition - the start of employment of a user in the organization)

## Queries

### Organization

Privilege level: Organization member
Basic informations about the organization - settings and name.

### Organization's users

Privilege level: organization member
List of organization's members

### Organization's Vacations

Privilege level: organization member
List of organization's vacations. Ability to specify as a parameter the date from which vacations in to the future will be returned.

### Vacation info

Privilege level: administrator or vacation owner
Returns vacations along with calculated statistics

### Calculate vacations

Privilege level: Administrator
Calculates statistics for a non-existent vacation that would be assigned to a given user.

### User

Privilege level: Administrator
Returns the user with the specified id.

### User Info

Privilege level: Administrator
Returns information about the user with the given id (user, vacation list, transition list and stats)

### Me

Privilege level: User
Returns the user who authorizes himself

### My Info

Authorization level: User
Returns information about the user who is authorizing (user, vacation list, transition list and stats)
