Map<String, dynamic> initialData = Map<String, dynamic>();

this.initialData = const {
        'Header': 'Match Scouting',
        'Team Number': 0,
'cool':'',
'yes':'',

//new question
'nice':0,
'very':0,

//new question
'mac':0,
'linux':0,

//new question
'Question':['answer2','answer1',],
'Question2':['answer2','answer1',],
//new question
'Question':['answer2','answer1',],
'Question2':['answer2','answer1',],
//new question
'Question':['answer2','answer1',],
'Question2':['answer2','answer1',],
};
List<Question>? matchFormQuestions;
matchFormQuestions = [ShortAnswer(
        'Team Number',
        TextInputType.number,
        initialValue: widget.initialData['Team Number'],
      ),
ShortAnswer(
'very',
TextInputType.number,
initialValue: widget.initialData['very'],
),
ShortAnswer(
'nice',
TextInputType.number,
initialValue: widget.initialData['nice'],
),
ShortAnswer(
'yes',
TextInputType.text,
initialValue: widget.initialData['yes'],
),
ShortAnswer(
'cool',
TextInputType.text,
initialValue: widget.initialData['cool'],
),
UpDownArrowQuestion('linux',
counter: widget.initialData['linux'],
),
UpDownArrowQuestion('mac',
counter: widget.initialData['mac'],
),
DropDownQuestion(
'Question',
['answer2','answer1',],
,answer: widget.initialData['Question']
),
DropDownQuestion(
'Question2',
['answer2','answer1',],
,answer: widget.initialData['Question2']
),
DropDownQuestion(
'Question',
['answer2','answer1',],
,answer: widget.initialData['Question']
),
DropDownQuestion(
'Question2',
['answer2','answer1',],
,answer: widget.initialData['Question2']
),
DropDownQuestion(
'Question',
['answer2','answer1',],
,answer: widget.initialData['Question']
),
DropDownQuestion(
'Question2',
['answer2','answer1',],
,answer: widget.initialData['Question2']
),
];
